use riot_sys as raw;
use riot_sys::libc;

use core::marker::PhantomData;

use core::intrinsics::transmute;

// // wrongly detected as u32, it's actually used as an i32
// pub const THREAD_CREATE_SLEEPING: i32 = 1;
// pub const THREAD_AUTO_FREE: i32 = 2;
// pub const THREAD_CREATE_WOUT_YIELD: i32 = 4;
// pub const THREAD_CREATE_STACKTEST: i32 = 8;
//
// // wrongly detected as u32, it's actually used as a u8
// pub const THREAD_PRIORITY_MIN: i8 = 15;
// pub const THREAD_PRIORITY_IDLE: i8 = 15;
// pub const THREAD_PRIORITY_MAIN: i8 = 7;

// Possible optimization: Make this NonZero
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct KernelPID(raw::kernel_pid_t);

pub(crate) mod pid_converted {
    //! Converting the raw constants into consistently typed ones
    use riot_sys as raw;

    // pub const KERNEL_PID_UNDEF: raw::kernel_pid_t = raw::KERNEL_PID_UNDEF as raw::kernel_pid_t;
    pub const KERNEL_PID_FIRST: raw::kernel_pid_t = raw::KERNEL_PID_FIRST as raw::kernel_pid_t;
    pub const KERNEL_PID_LAST: raw::kernel_pid_t = raw::KERNEL_PID_LAST as raw::kernel_pid_t;
    pub const KERNEL_PID_ISR: raw::kernel_pid_t = raw::KERNEL_PID_ISR as raw::kernel_pid_t;
}

mod status_converted {
    //! Converting the raw constants into consistently typed ones for use in match branches. If
    //! that becomes a pattern, it might make sense to introduce a macro that forces a bunch of
    //! symbols (with different capitalizations) into a given type and makes an enum with a
    //! from_int method out of it.

    use riot_sys as raw;

    // STATUS_NOT_FOUND is not added here as it's not a proper status but rather a sentinel value,
    // which moreover can't be processed in its current form by bindgen and would need to be copied
    // over in here by manual expansion of the macro definition.
    pub const STATUS_STOPPED: i32 = raw::thread_status_t_STATUS_STOPPED as i32;
    pub const STATUS_SLEEPING: i32 = raw::thread_status_t_STATUS_SLEEPING as i32;
    pub const STATUS_MUTEX_BLOCKED: i32 = raw::thread_status_t_STATUS_MUTEX_BLOCKED as i32;
    pub const STATUS_RECEIVE_BLOCKED: i32 = raw::thread_status_t_STATUS_RECEIVE_BLOCKED as i32;
    pub const STATUS_SEND_BLOCKED: i32 = raw::thread_status_t_STATUS_SEND_BLOCKED as i32;
    pub const STATUS_REPLY_BLOCKED: i32 = raw::thread_status_t_STATUS_REPLY_BLOCKED as i32;
    pub const STATUS_FLAG_BLOCKED_ANY: i32 = raw::thread_status_t_STATUS_FLAG_BLOCKED_ANY as i32;
    pub const STATUS_FLAG_BLOCKED_ALL: i32 = raw::thread_status_t_STATUS_FLAG_BLOCKED_ALL as i32;
    pub const STATUS_MBOX_BLOCKED: i32 = raw::thread_status_t_STATUS_MBOX_BLOCKED as i32;
    pub const STATUS_RUNNING: i32 = raw::thread_status_t_STATUS_RUNNING as i32;
    pub const STATUS_PENDING: i32 = raw::thread_status_t_STATUS_PENDING as i32;
}

#[derive(Debug)]
pub enum Status {
    // I would not rely on any properties of the assigned values, but it might make the conversion
    // points easier on the generated code if it can be reasoned down to a simple check of whether
    // it's in range.
    Stopped = status_converted::STATUS_STOPPED as isize,
    Sleeping = status_converted::STATUS_SLEEPING as isize,
    MutexBlocked = status_converted::STATUS_MUTEX_BLOCKED as isize,
    ReceiveBlocked = status_converted::STATUS_RECEIVE_BLOCKED as isize,
    SendBlocked = status_converted::STATUS_SEND_BLOCKED as isize,
    ReplyBlocked = status_converted::STATUS_REPLY_BLOCKED as isize,
    FlagBlockedAny = status_converted::STATUS_FLAG_BLOCKED_ANY as isize,
    FlagBlockedAll = status_converted::STATUS_FLAG_BLOCKED_ALL as isize,
    MboxBlocked = status_converted::STATUS_MBOX_BLOCKED as isize,
    Running = status_converted::STATUS_RUNNING as isize,
    Pending = status_converted::STATUS_PENDING as isize,

    Other, // Not making this Other(i32) as by the time this is reached, the code can't react
           // meaningfully to it, and if that shows up in any debug output, someone will need to
           // reproduce this anyway and can hook into from_int then.
}

impl Status {
    pub fn is_on_runqueue(&self) -> bool {
        // FIXME: Why don't I get STATUS_ON_RUNQUEUE? Without that, I can just as well check for
        // being either or.
        match self {
            Status::Pending => true,
            Status::Running => true,
            _ => false,
        }
    }

    fn from_int(status: i32) -> Self {
        match status {
            status_converted::STATUS_STOPPED => Status::Stopped,
            status_converted::STATUS_SLEEPING => Status::Sleeping,
            status_converted::STATUS_MUTEX_BLOCKED => Status::MutexBlocked,
            status_converted::STATUS_RECEIVE_BLOCKED => Status::ReceiveBlocked,
            status_converted::STATUS_SEND_BLOCKED => Status::SendBlocked,
            status_converted::STATUS_REPLY_BLOCKED => Status::ReplyBlocked,
            status_converted::STATUS_FLAG_BLOCKED_ANY => Status::FlagBlockedAny,
            status_converted::STATUS_FLAG_BLOCKED_ALL => Status::FlagBlockedAll,
            status_converted::STATUS_MBOX_BLOCKED => Status::MboxBlocked,
            status_converted::STATUS_RUNNING => Status::Running,
            status_converted::STATUS_PENDING => Status::Pending,
            _ => Status::Other,
        }
    }
}

impl KernelPID {
    pub fn new(pid: raw::kernel_pid_t) -> Option<Self> {
        // from static inline pid_is_valid
        // casts needed due to untypedness of preprocessor constants
        if pid >= pid_converted::KERNEL_PID_FIRST && pid <= pid_converted::KERNEL_PID_LAST {
            Some(KernelPID(pid))
        } else {
            None
        }
    }

    pub fn all_pids() -> impl Iterator<Item = KernelPID> {
        (pid_converted::KERNEL_PID_FIRST..=pid_converted::KERNEL_PID_LAST).map(|i| KernelPID(i))
    }

    pub fn get_name(&self) -> Option<&str> {
        let ptr = unsafe { raw::thread_getname(self.0) };
        if ptr == 0 as *const libc::c_char {
            return None;
        }
        // If the thread stops, the name might be not valid any more, but then again the getname
        // function might already have returned anything, and thread names are generally strings in
        // .text. Unwrapping because by the time non-ASCII text shows up in there, something
        // probably already went terribly wrong.
        let name: &str = unsafe { libc::CStr::from_ptr(ptr) }.to_str().unwrap();
        Some(name)
    }

    pub fn get_status(&self) -> Status {
        let status = unsafe { raw::thread_getstatus(self.0) };
        Status::from_int(status)
    }

    pub fn wakeup(&self) -> Result<(), ()> {
        let success = unsafe { raw::thread_wakeup(self.0) };
        match success {
            1 => Ok(()),
            // Actuall STATUS_NOT_FOUND, but all the others are then all error cases.
            _ => Err(()),
        }
    }
}

impl Into<raw::kernel_pid_t> for &KernelPID {
    fn into(self) -> raw::kernel_pid_t {
        self.0
    }
}

impl Into<raw::kernel_pid_t> for KernelPID {
    fn into(self) -> raw::kernel_pid_t {
        self.0
    }
}

pub fn get_pid() -> KernelPID {
    // implementing the static thread_getpid function:
    KernelPID(unsafe { ::core::ptr::read_volatile(&raw::sched_active_pid) })
}

pub fn sleep() {
    unsafe { raw::thread_sleep() }
}




/// Internal helper that does all the casting but relies on the caller to establish appropriate
/// lifetimes.
///
/// This also returns a pointer to the created thread's control block inside the stack; that TCB
/// can be used to get the thread's status even when the thread is already stopped and the PID may
/// have been reused for a different thread. For short-lived threads that are done before this
/// function returns, the TCB may be None.
unsafe fn create<R>(
        stack: &mut [u8],
        closure: &mut R,
        name: &libc::CStr,
        priority: i8,
        flags: i32,
) -> (raw::kernel_pid_t, Option<*mut riot_sys::_thread>)
where
    R: Send + FnMut(),
{
    // overwriting name "R" as suggested as "copy[ing] over the parameters" on
    // https://doc.rust-lang.org/error-index.html#E0401
    unsafe extern "C" fn run<R>(x: *mut libc::c_void) -> *mut libc::c_void
    where
        R: Send + FnMut(),
    {
        let closure: &mut R = transmute(x);
        closure();
        0 as *mut libc::c_void
    }

    let pid = raw::thread_create(
        transmute(stack.as_mut_ptr()),
        stack.len() as i32,
        priority,
        flags,
        Some(run::<R>),
        closure as *mut R as *mut _,
        name.as_ptr(),
    );

    let tcb = riot_sys::thread_get(pid);
    // FIXME: Rather than doing pointer comparisons, it'd be nicer to just get the stack's
    // calculated thread control block (TCB) position and look right in there.
    let tcb = if tcb >= &stack[0] as *const u8 as *mut _
        && tcb <= &stack[stack.len() - 1] as *const u8 as *mut _
    {
        Some(tcb)
    } else {
        None
    };

    (pid, tcb)
}

/// Create a context for starting threads that take shorter than 'static references.
///
/// Inside the scope, threads can be created using the `.spawn()` method of the scope passed in,
/// similar to the scoped-threads RFC (which resembles crossbeam's threads). Unlike that, the scope
/// has no dynamic memory of the spawned threads, and no actual way of waiting for a thread. If the
/// callback returns, the caller has call the scope's `.reap()` method with all the threads that
/// were launched; otherwise, the program panics.
pub fn scope<F>(callback: F)
where
    F: FnOnce(&mut CountingThreadScope)
{
    let mut s = CountingThreadScope { threads: 0 };

    callback(&mut s);

    s.wait_for_all();
}

pub struct CountingThreadScope {
    threads: u16, // a counter, but larger than kernel_pid_t
}

impl CountingThreadScope {
    /// Start a thread in the given stack, in which the closure is run. The thread gets a human
    /// readable name (ignored in no-DEVHELP mode), and is started with the priority and flags as
    /// per thread_create documentation.
    ///
    /// The returned thread object can safely be discarded when the scope is not expected to ever
    /// return, and needs to be passed on to `.reap()` otherwise.
    ///
    /// Having the closure as a mutable reference (rather than a moved instance) is a bit
    /// unergonomic as it means that `spawn(..., || { foo }, ..)` one-line invocations are
    /// impossible, but is necessary as it avoids having the callback sitting in the Thread which
    /// can't be prevented from moving around on the stack between the point when thread_create is
    /// called (and the pointer is passed on to RIOT) and the point when the threads starts running
    /// and that pointer is used.
    pub fn spawn<'scope, 'pieces, R>(&'scope mut self,
        stack: &'pieces mut [u8],
        closure: &'pieces mut R,
        name: &'pieces libc::CStr,
        priority: i8,
        flags: i32,
    ) -> Result<CountedThread<'pieces>, raw::kernel_pid_t>
    where
        'pieces: 'scope,
        R: Send + FnMut(),
    {
        self.threads = self.threads.checked_add(1).expect("Thread limit exceeded");

        let (pid, tcb) = unsafe { create(stack, closure, name, priority, flags) };

        if pid < 0 {
            return Err(pid);
        }

        Ok(CountedThread {
            thread: TrackedThread {
                pid: KernelPID(pid),
                tcb: tcb,
            },
            _phantom: PhantomData,
        })
    }

    /// Assert that the thread has terminated, and remove it from the list of pending threads in
    /// this context.
    ///
    /// Unlike a (POSIX) wait, this will not block (for there is no SIGCHLDish thing in RIOT --
    /// whoever wants to be notified would need to make their threads send an explicit signal), but
    /// panic if the thread is not actually done yet.
    pub fn reap(&mut self, thread: CountedThread) {
        // FIXME: check whether the counted thread 
        match thread.get_status() {
            Status::Stopped => (),
            _ => panic!("Attempted to reap running process"),
        }

        self.threads -= 1;
    }

    fn wait_for_all(self) {
        if self.threads != 0 {
            panic!("Not all threads were waited for at scope end");
        }
    }
}

// The 'pieces should (FIXME: verify) help ensuring that threads can only be reaped where they were
// created. (It might make sense to move it into TrackedThread and make the tcb usable for more
// than just pointer comparison).
pub struct CountedThread<'pieces> {
    thread: TrackedThread,
    _phantom: PhantomData<&'pieces ()>
}

impl<'pieces> CountedThread<'pieces> {
    pub fn get_pid(&self) -> KernelPID {
        self.thread.get_pid()
    }

    pub fn get_status(&self) -> Status {
        self.thread.get_status()
    }
}


pub fn spawn<R>(
        stack: &'static mut [u8],
        closure: &'static mut R,
        name: &'static libc::CStr,
        priority: i8,
        flags: i32,
    ) -> Result<TrackedThread, raw::kernel_pid_t>
where
    R: Send + FnMut(),
{
    let (pid, tcb) = unsafe { create(stack, closure, name, priority, flags) };

    if pid < 0 {
        return Err(pid);
    }

    Ok(TrackedThread { pid: KernelPID(pid), tcb })
}

/// A thread identified not only by its PID (which can be reused whenever the thread has quit) but
/// also by a pointer to its thread control block. This gives a TrackedThread a better get_status()
/// method that reliably reports Stopped even when the PID is reused.
///
/// A later implementation may stop actually having the pid in the struct and purely rely on the
/// tcb (although that'll need to become a lifetime'd reference to a cell by then).
pub struct TrackedThread {
    pid: KernelPID,
    tcb: Option<*mut riot_sys::_thread>,
}

impl TrackedThread {
    pub fn get_pid(&self) -> KernelPID {
        self.pid
    }

    /// Like get_status of a KernelPID, but this returnes Stopped if the PID has been re-used after
    /// our thread has stopped.
    pub fn get_status(&self) -> Status {
        let status = self.pid.get_status();
        let tcb = unsafe { riot_sys::thread_get(self.pid.0) };
        if Some(tcb) != self.tcb {
            Status::Stopped
        } else {
            status
        }
    }
}
