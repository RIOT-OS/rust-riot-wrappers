use riot_sys as raw;
use riot_sys::libc;

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
#[derive(Debug, PartialEq)]
pub struct KernelPID(pub raw::kernel_pid_t);

mod status_converted {
    //! Converting the raw constants into consistently typed ones for use in match branches. If
    //! that becomes a pattern, it might make sense to introduce a macro that forces a bunch of
    //! symbols (with different capitalizations) into a given type and makes an enum with a
    //! from_int method out of it.

    use riot_sys as raw;

    pub const STATUS_NOT_FOUND: i32 = raw::STATUS_NOT_FOUND as i32;
    pub const STATUS_STOPPED: i32 = raw::STATUS_STOPPED as i32;
    pub const STATUS_SLEEPING: i32 = raw::STATUS_SLEEPING as i32;
    pub const STATUS_MUTEX_BLOCKED: i32 = raw::STATUS_MUTEX_BLOCKED as i32;
    pub const STATUS_RECEIVE_BLOCKED: i32 = raw::STATUS_RECEIVE_BLOCKED as i32;
    pub const STATUS_SEND_BLOCKED: i32 = raw::STATUS_SEND_BLOCKED as i32;
    pub const STATUS_REPLY_BLOCKED: i32 = raw::STATUS_REPLY_BLOCKED as i32;
    pub const STATUS_FLAG_BLOCKED_ANY: i32 = raw::STATUS_FLAG_BLOCKED_ANY as i32;
    pub const STATUS_FLAG_BLOCKED_ALL: i32 = raw::STATUS_FLAG_BLOCKED_ALL as i32;
    pub const STATUS_MBOX_BLOCKED: i32 = raw::STATUS_MBOX_BLOCKED as i32;
    pub const STATUS_RUNNING: i32 = raw::STATUS_RUNNING as i32;
    pub const STATUS_PENDING: i32 = raw::STATUS_PENDING as i32;
}

#[derive(Debug)]
pub enum Status {
    // I would not rely on any properties of the assigned values, but it might make the conversion
    // points easier on the generated code if it can be reasoned down to a simple check of whether
    // it's in range.
    NotFound = status_converted::STATUS_NOT_FOUND as isize,
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
             status_converted::STATUS_NOT_FOUND => Status::NotFound,
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

impl KernelPID
{
    pub fn get_name(&self) -> Option<&str>
    {
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

    pub fn get_status(&self) -> Status
    {
        let status = unsafe { raw::thread_getstatus(self.0) };
        Status::from_int(status)
    }

    pub fn wakeup(&self) -> Result<(), ()>
    {
        let success = unsafe { raw::thread_wakeup(self.0) };
        match success {
            1 => Ok(()),
            // Actuall STATUS_NOT_FOUND, but all the others are then all error cases.
            _ => Err(())
        }
    }
}

pub fn get_pid() -> KernelPID
{
    // implementing the static thread_getpid function:
    KernelPID(unsafe { ::core::ptr::read_volatile(&raw::sched_active_pid) })
}

pub fn sleep()
{
    unsafe { raw::thread_sleep() }
}

pub struct Thread<'a, R> {
    name: &'a libc::CStr,
    stack: &'a mut [u8],
    closure: R,
}

impl<'a, R> Thread<'a, R>
    where R: Send + FnMut(),
{
    pub fn prepare(
            stack: &'a mut [u8],
            closure: R,
            name: &'a libc::CStr,
        ) -> Self
    {
        Thread { stack, name, closure }
    }

    // As with saul::ContainedRegistration start, we can't do this in the original constructor as
    // the value will still move around. Like there, I'd actually need to pin self, for it could
    // move around between the invocation of start (where the pointer to self or self.closure is
    // passed out to the OS thread) and the start of the thread, which for low-priorized or
    // WOUT_YIELD threads can be a later time.
    pub fn start(
        &mut self,
        priority: i8,
        flags: i32,
    ) -> KernelPID {
        let pid = unsafe { raw::thread_create(
            transmute(self.stack.as_mut_ptr()), self.stack.len() as i32,
            priority,
            flags,
            Some(Self::run),
            transmute(&mut self.closure),
            self.name.as_ptr(),
            ) };
        KernelPID(pid)
    }

    unsafe extern "C" fn run(x: *mut libc::c_void) -> *mut libc::c_void {
        let closure: &mut R = transmute(x);
        closure();
        0 as *mut libc::c_void
    }
}

impl<'a, R> Drop for Thread<'a, R> {
    fn drop(&mut self) {
        // We don't even know whether it has been started, either
        panic!("Can't drop a Thread because I can't kill the process (or make sure it has died for good)");
    }
}
