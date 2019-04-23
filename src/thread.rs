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

// FIXME: The argument should not be pub, the constructor should check the range and eg. reject
// building one with KERNEL_PID_ISR.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct KernelPID(pub raw::kernel_pid_t);

mod status_converted {
    //! Converting the raw constants into consistently typed ones for use in match branches. If
    //! that becomes a pattern, it might make sense to introduce a macro that forces a bunch of
    //! symbols (with different capitalizations) into a given type and makes an enum with a
    //! from_int method out of it.

    use riot_sys as raw;

    // STATUS_NOT_FOUND is not added here as it's not a proper status but rather a sentinel value,
    // which moreover can't be processed in its current form by bindgen and would need to be copied
    // over in here by manual expansion of the macro definition.
    pub const STATUS_STOPPED: i32 = raw::thread_state_t_STATUS_STOPPED as i32;
    pub const STATUS_SLEEPING: i32 = raw::thread_state_t_STATUS_SLEEPING as i32;
    pub const STATUS_MUTEX_BLOCKED: i32 = raw::thread_state_t_STATUS_MUTEX_BLOCKED as i32;
    pub const STATUS_RECEIVE_BLOCKED: i32 = raw::thread_state_t_STATUS_RECEIVE_BLOCKED as i32;
    pub const STATUS_SEND_BLOCKED: i32 = raw::thread_state_t_STATUS_SEND_BLOCKED as i32;
    pub const STATUS_REPLY_BLOCKED: i32 = raw::thread_state_t_STATUS_REPLY_BLOCKED as i32;
    pub const STATUS_FLAG_BLOCKED_ANY: i32 = raw::thread_state_t_STATUS_FLAG_BLOCKED_ANY as i32;
    pub const STATUS_FLAG_BLOCKED_ALL: i32 = raw::thread_state_t_STATUS_FLAG_BLOCKED_ALL as i32;
    pub const STATUS_MBOX_BLOCKED: i32 = raw::thread_state_t_STATUS_MBOX_BLOCKED as i32;
    pub const STATUS_RUNNING: i32 = raw::thread_state_t_STATUS_RUNNING as i32;
    pub const STATUS_PENDING: i32 = raw::thread_state_t_STATUS_PENDING as i32;
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
    pub fn all_pids() -> impl Iterator<Item = KernelPID> {
        (raw::KERNEL_PID_FIRST as i16..raw::KERNEL_PID_LAST as i16).map(|i| KernelPID(i))
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

pub fn get_pid() -> KernelPID {
    // implementing the static thread_getpid function:
    KernelPID(unsafe { ::core::ptr::read_volatile(&raw::sched_active_pid) })
}

pub fn sleep() {
    unsafe { raw::thread_sleep() }
}

pub struct Thread<'a, R> {
    stack: &'a mut [u8],
    // No need to store the actual closure; we've already taken a reference that lives long enough
    // at spawn time. We still need to be generic on R to have an adaequate `run` method that can
    // be function-pointed to.
    _closure: PhantomData<R>,
    // FIXME: Once we can calculate the TCB, we don't need to pass this around any more.
    pid: KernelPID,
}

impl<'a, R> Thread<'a, R>
where
    R: Send + FnMut(),
{
    /// Start a thread in the given stack, in which the closure is run. The thread gets a human
    /// readable name (ignored in no-DEVHELP mode), and is started with the priority and flags as
    /// per thread_create documentation.
    ///
    /// The returned thread object keeps a reference to the stack to ensure its immutability.
    /// Contrary to the usual Rust model (i.o.w. FIXME until Rust has linear types, or one needs to
    /// constrain 'a to 'static), the user needs to make sure that its destructor is run by not
    /// forgetting about the thread.
    ///
    /// Having the closure as a mutable reference (rather than a moved instance) is a bit
    /// unergonomic as it means that `spawn(..., || { foo }, ..)` one-line invocations are
    /// impossible, but is necessary as it avoids having the callback sitting in the Thread which
    /// can't be prevented from moving around on the stack between the point when thread_create is
    /// called (and the pointer is passed on to RIOT) and the point when the threads starts running
    /// and that pointer is used. (That was also an issue with the previous design that had a
    /// .prepare() constructor and a .run() activator. It might be salvagable by having a prepared
    /// Thread and an undroppable returned ThreadHandle that contains a references from .run(), but
    /// that appears to be overly complicated right now).
    #[deprecated(note="This interface repeats the leakpocalypse, use .scope() instead")]
    pub fn spawn(
        stack: &'a mut [u8],
        closure: &'a mut R,
        name: &'a libc::CStr,
        priority: i8,
        flags: i32,
    ) -> Result<Self, raw::kernel_pid_t> {
        let pid = unsafe {
            raw::thread_create(
                transmute(stack.as_mut_ptr()),
                stack.len() as i32,
                priority,
                flags,
                Some(Self::run),
                transmute(closure),
                name.as_ptr(),
            )
        };

        if pid < 0 {
            return Err(pid);
        }

        Ok(Thread {
            stack,
            _closure: PhantomData,
            pid: KernelPID(pid),
        })
    }

    unsafe extern "C" fn run(x: *mut libc::c_void) -> *mut libc::c_void {
        let closure: &mut R = transmute(x);
        closure();
        0 as *mut libc::c_void
    }

    pub fn get_pid(&self) -> KernelPID {
        self.pid
    }

    /// Like get_status of a KernelPID, but this returnes Stopped if the PID has been re-used after
    /// our thread has stopped.
    // Still can't implement a good `wait` with this for lack of a blocking condition, but at least
    // now we can check half-assedly (see TCB comment) whether what's running at our PID is
    // actually still our process. A proper wait would need the wrapped closure to finally send
    // some kind of signal to something that's allocated ... somewhere located within the Thread
    // struct that's not known by the time the thread is spawned.
    pub fn get_status(&self) -> Status {
        let running_status = self.pid.get_status();
        let running_tcb = unsafe { riot_sys::thread_get(self.pid.0) };

        // FIXME: Rather than doing pointer comparisons, it'd be nicer to just get the stack's
        // calculated thread control block (TCB) position and look right in there.
        if running_tcb >= &self.stack[0] as *const u8 as *mut _
            && running_tcb <= &self.stack[self.stack.len() - 1] as *const u8 as *mut _
        {
            running_status
        } else {
            Status::Stopped
        }
    }

    /// Assert that the thread has terminated, and release all references by consuming the self
    /// struct.
    ///
    /// Unlike a (POSIX) wait, this will not block (for there is no SIGCHLDish thing in RIOT --
    /// whoever wants to be notified would need to make their threads send an explicit signal), but
    /// panic if the thread is not actually done yet.
    pub fn reap(self) {
        match self.get_status() {
            Status::Stopped => core::mem::forget(self),
            _ => panic!("Attempted to reap running process"),
        }
    }
}

impl<'a, R> Drop for Thread<'a, R> {
    fn drop(&mut self) {
        // We don't even know whether it has been started, either
        panic!("Can't drop a Thread because I can't kill the process (or make sure it has died for good)");
    }
}


pub fn scope<F>(callback: F)
where
    F: FnOnce(&mut ThreadScope)
{
    let mut s = ThreadScope { _private: () };

    callback(&mut s);

    s.wait_for_all();
}

pub struct ThreadScope {
    _private: ()
}

impl ThreadScope {
    /// Start a thread in the given stack, in which the closure is run. The thread gets a human
    /// readable name (ignored in no-DEVHELP mode), and is started with the priority and flags as
    /// per thread_create documentation.
    ///
    /// The returned thread object can safely be discarded; 
    ///
    /// Having the closure as a mutable reference (rather than a moved instance) is a bit
    /// unergonomic as it means that `spawn(..., || { foo }, ..)` one-line invocations are
    /// impossible, but is necessary as it avoids having the callback sitting in the Thread which
    /// can't be prevented from moving around on the stack between the point when thread_create is
    /// called (and the pointer is passed on to RIOT) and the point when the threads starts running
    /// and that pointer is used. (That was also an issue with the previous design that had a
    /// .prepare() constructor and a .run() activator. It might be salvagable by having a prepared
    /// Thread and an undroppable returned ThreadHandle that contains a references from .run(), but
    /// that appears to be overly complicated right now).
    pub fn spawn<'scope, 'pieces, R>(&'scope mut self,
        stack: &'pieces mut [u8],
        closure: &'pieces mut R,
        name: &'pieces libc::CStr,
        priority: i8,
        flags: i32,
    ) -> Result<SpawnedThread<'scope>, raw::kernel_pid_t>
    where
        'pieces: 'scope,
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

        let pid = unsafe {
            raw::thread_create(
                transmute(stack.as_mut_ptr()),
                stack.len() as i32,
                priority,
                flags,
                Some(run::<R>),
                transmute(closure),
                name.as_ptr(),
            )
        };

        if pid < 0 {
            return Err(pid);
        }

        Ok(SpawnedThread {
            pid: KernelPID(pid),
            _phantom: PhantomData,
        })
    }

    fn wait_for_all(self) {
        // a) because not even threads can be waited for, and
        // b) a ThreadScope doesn't have the dynamic memory to keep track of all N threads it may
        // have spawned.
        //
        // This could be enhanced in the future by at least counting the number of spawned threads
        // (in constant memory), and panicing unless all the spawned threads have been shut down
        // properly beforehand.
        panic!("Can not wait for all threads");
    }
}

// The 'scope is currently not necessary, but I anticipate right now that it'll be needed if and
// when it contains a TCB reference to adaequately query the thread's status.
pub struct SpawnedThread<'scope> {
    pid: KernelPID,
    _phantom: PhantomData<&'scope ()>
}

impl<'scope> SpawnedThread<'scope> {
    pub fn get_pid(&self) -> KernelPID {
        self.pid
    }
}
