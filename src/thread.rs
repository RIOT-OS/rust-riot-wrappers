// Manually adapted from the output of
//
//     bindgen ../RIOT/core/include/thread.h --whitelist-function thread_create --use-core -o thread.rs -- -I ../RIOT/sys/include -I ../RIOT/drivers/include -I ../RIOT/core/include -I .
//
// with internal int definitions and documentation extracts dropped.

use libc;

pub type kernel_pid_t = i16;
pub type thread_task_func_t =
    ::core::option::Option<unsafe extern "C" fn(arg: *mut libc::c_void) -> *mut libc::c_void>;
extern "C" {
    pub fn thread_create(
        stack: *mut libc::c_char,
        stacksize: libc::c_int,
        priority: libc::c_char,
        flags: libc::c_int,
        task_func: thread_task_func_t,
        arg: *mut libc::c_void,
        name: *const libc::c_char,
    ) -> kernel_pid_t;
    pub fn thread_wakeup(pid: kernel_pid_t) -> libc::c_int;
}

// manually added from whitelist-less output, didn't find how to whitelist them

// wrongly detected as u32, it's actually used as an i32
pub const THREAD_CREATE_SLEEPING: i32 = 1;
pub const THREAD_AUTO_FREE: i32 = 2;
pub const THREAD_CREATE_WOUT_YIELD: i32 = 4;
pub const THREAD_CREATE_STACKTEST: i32 = 8;

// wrongly detected as u32, it's actually used as a u8
pub const THREAD_PRIORITY_MIN: i8 = 15;
pub const THREAD_PRIORITY_IDLE: i8 = 15;
pub const THREAD_PRIORITY_MAIN: i8 = 7;
