use raw;
use libc;

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

#[derive(Debug)]
pub struct KernelPID(pub raw::kernel_pid_t);

impl KernelPID
{
    pub fn getname(&self) -> Option<&str>
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

pub fn getpid() -> KernelPID
{
//     KernelPID(raw::thread_getpid())
    unimplemented!("That's a static function in C");
}

pub fn sleep()
{
    unsafe { raw::thread_sleep() }
}
