// Manually adapted from the output of
//
//     bindgen ../RIOT/sys/include/shell.h --use-core -o shell.rs -- -I ../RIOT/sys/include -I ../RIOT/drivers/include -I ../RIOT/core/include
//
// (no -I . needed) with some cherry-picking.

use libc;

pub type shell_command_handler_t = ::core::option::Option<
    unsafe extern "C" fn(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct shell_command_t {
    pub name: *const libc::c_char,
    pub desc: *const libc::c_char,
    pub handler: shell_command_handler_t,
}

extern "C" {
    // changed: function diverges as per documentation.
    pub fn shell_run(
        commands: *const shell_command_t,
        line_buf: *mut libc::c_char,
        len: libc::c_int,
    ) -> !;
}

// changed: usize
pub const SHELL_DEFAULT_BUFSIZE: usize = 128;


// This won't go into the sys crate, but into the high-level-abstractions of it

// not repr(C) for as long as run() copies over all the inner commands, but there might be a time
// when we pack it into something null-terminatable from the outside and then repr(C) would help
// again. -- well actually it seems that we *need* to have an additional slot in here where to
// store the closure, and don't need to store the full struct. -- well again not right now, as
// acting on the closure would need a userdata argument which is not there (cf. freenode/#rust
// 2018-02-21 14:30CEST), so passing around callbacks directly.
#[derive(Copy, Clone)]
pub struct ShellCommand<'a>
{
    name: &'a libc::CStr,
    desc: &'a libc::CStr,
    handler: unsafe extern "C" fn(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int,
}

impl<'a> ShellCommand<'a>
{
    pub fn new(name: &'a libc::CStr, desc: &'a libc::CStr, handler: unsafe extern "C" fn(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int) -> Self
    {
        ShellCommand {
            name: name,
            desc: desc,
            handler: handler,
        }
    }

    pub fn as_shell_command(&self) -> shell_command_t
    {
        shell_command_t {
            name: self.name.as_ptr(),
            desc: self.desc.as_ptr(),
            handler: Some(self.handler),
        }
    }
}

fn null_shell_command() -> shell_command_t
{
    shell_command_t { 
            name: 0 as *const libc::c_char,
            desc: 0 as *const libc::c_char,
            handler: None,
    }
}

pub fn run(command: &ShellCommand, line_buf: &mut[u8]) -> !
{
    const LIMIT: usize = 5;
    // FIXME: Arbitrary size limit, find an idiom to pass in a null-terminated slice or to allocate
    // a variable-lenth (commands.len() + 1) structure on the stack.
    let mut args: [shell_command_t; LIMIT + 1] = [null_shell_command(); LIMIT + 1];

    // Having commands has become difficult when ShellCommand became ShellCommand<H>; using a
    // single command to begin with.
//     if commands.len()  > LIMIT {
//         panic!("Static command count exceeded");
//     }
// 
//     for (src, dest) in commands.iter().zip(&mut args[..LIMIT]) {
//         *dest = src.internal_command;
//     }

    args[0] = command.as_shell_command();

    unsafe { shell_run(
            args.as_ptr(),
            line_buf.as_mut_ptr() as *mut i8,
            line_buf.len() as i32, // FIXME: panic if len is too large
            )
    }
}

/// Take the passed on arguments of a shell_command_handler_t and call an inner function that
/// receives those arguments in nice str slice form.
pub fn command_wrap_inner<F>(argc: libc::c_int, argv: *mut *mut libc::c_char, inner: F) -> i32
where F: Fn(&[&str]) -> i32
{
    // Same issue as with run, see LIMIT there
    const LIMIT: usize = 5;
    let mut args: [&str; LIMIT] = [&""; 5];

    let argc: usize = if argc < 0 { 0 } else if argc as usize > LIMIT { LIMIT } else { argc as usize };

    let argv: *mut *mut u8 = unsafe { ::core::mem::transmute(argv) };
    let argv: &[*mut u8] = unsafe { ::core::slice::from_raw_parts(argv, argc) };

    for i in 0..argc {
        let start = argv[i];
        // I *really* need a no_std CStr...
        let mut slice = unsafe { ::core::slice::from_raw_parts(start, 1) };
        loop {
            if slice[slice.len() - 1] == 0 {
                slice = unsafe { ::core::slice::from_raw_parts(start, slice.len() - 1) };
                break;
            } else {
                slice = unsafe { ::core::slice::from_raw_parts(start, slice.len() + 1) };
            }
        }
        args[i] = ::core::str::from_utf8(slice).unwrap();
    }

    inner(&args[..argc])
}
