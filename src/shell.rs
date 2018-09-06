use riot_sys::libc;
use stdio;
use core::fmt::Write;
use riot_sys::{
    shell_run,
    shell_command_t,
};


// not repr(C) for as long as run() copies over all the inner commands, but there might be a time
// when we pack it into something null-terminatable from the outside and then repr(C) would help
// again. -- well actually it seems that we *need* to have an additional slot in here where to
// store the closure, and don't need to store the full struct. -- well again not right now, as
// acting on the closure would need a userdata argument which is not there (cf. freenode/#rust
// 2018-02-21 14:30CEST), so passing around callbacks directly.
#[derive(Copy, Clone)]
pub struct ShellCommand<'a, R>
{
    name: &'a libc::CStr,
    desc: &'a libc::CStr,
    handler: R,
}

impl<'a, R> ShellCommand<'a, R>
{
    unsafe extern "C" fn execute(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
        let commands = steal_global_run_state();
        for c in commands.iter_mut() {
            if let Some(result) = c.try_run(argc, argv) {
                return result;
            }
        }
        panic!("Command handler executed, but argv[0] does not match any known command");
    }

    pub fn new(name: &'a libc::CStr, desc: &'a libc::CStr, handler: R) -> Self
    {
        ShellCommand { name, desc, handler }
    }
}

// Only implemented as a trait so there can be trait object references in the GLOBAL_RUN_STATE
pub trait ShellCommandTrait {
    fn as_shell_command(&self) -> shell_command_t;

    /// If `argv[0]` matches the command's command name, run it and return some result; otherwise
    /// do nothing and return None.
    fn try_run(&mut self, argc: libc::c_int, argv: *mut *mut libc::c_char) -> Option<libc::c_int>;
}

impl<'a, 's, R> ShellCommandTrait for ShellCommand<'a, R>
    where R: FnMut(&[&str]) -> i32,
{
    fn as_shell_command(&self) -> shell_command_t
    {
        shell_command_t {
            name: self.name.as_ptr(),
            desc: self.desc.as_ptr(),
            handler: Some(Self::execute),
        }
    }

    fn try_run(&mut self, argc: libc::c_int, argv: *mut *mut libc::c_char) -> Option<libc::c_int>
    {
        let argv: &[*mut i8] = unsafe { ::core::slice::from_raw_parts(argv, argc as usize) };
        let marker = ();

        // This would save us the LIMIT, but I can't yet say
        //     where R: Fn(impl Iterator<Item=&str>>) -> i32
        // (yet?)
        // let argv = argv.iter().map(|ptr| unsafe { libc::CStr::from_ptr_with_lifetime(*ptr, &marker) }.to_bytes()).peekable();
        //
        // Instead, using a LIMIT:

        // Same issue as with run, see LIMIT there
        const LIMIT: usize = 10;
        let mut arg_array: [&str; LIMIT] = [&""; LIMIT];
        if argc > LIMIT as i32 {
            let mut stdio = stdio::Stdio {};
            // Might not even be my own handler, but as long as everyone has the same limit, why
            // not err out early.
            writeln!(stdio, "Not processing: too many arguments");
            return Some(1);
        }
        let argc = argc as usize;
        for i in 0..argc {
            arg_array[i] = unsafe { libc::CStr::from_ptr_with_lifetime(argv[i], &marker) }.to_str().unwrap();
        }
        let argv = &arg_array[..argc];

        if argv[0].as_bytes() == self.name.to_bytes() {
            let h = &mut self.handler;
            Some(h(argv))
        } else {
            None
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

static mut GLOBAL_RUN_STATE: usize = 0;
/// This is a brutal workaround for shell commands not being passed any additional data.
///
/// The function hands any caller an immutable reference to the shared location, under the
/// (invalid) assumption that there will only ever be one shell instance running and that won't
/// cross thread boundaries.
fn steal_global_run_state<'a>() -> &'a mut &'a mut [&'a mut dyn ShellCommandTrait] {
    unsafe { ::core::mem::transmute(GLOBAL_RUN_STATE as *const libc::c_void as *const _) }
}

pub fn run(commands: &[&mut dyn ShellCommandTrait], line_buf: &mut[u8]) -> !
{
    const LIMIT: usize = 5;
    // FIXME: Arbitrary size limit, find an idiom to pass in a null-terminated slice or to allocate
    // a variable-lenth (commands.len() + 1) structure on the stack. Possibly const numeric
    // generics will solve this.
    let mut args: [shell_command_t; LIMIT + 1] = [null_shell_command(); LIMIT + 1];

    if commands.len()  > LIMIT {
        panic!("Static command count exceeded");
    }

    for (src, dest) in commands.iter().zip(&mut args[..LIMIT]) {
        *dest = src.as_shell_command();
    }

    unsafe {
        if GLOBAL_RUN_STATE != 0 { panic!("Shell run more than once.") };
        GLOBAL_RUN_STATE = ::core::mem::transmute(&commands);
    }

    unsafe { shell_run(
            args.as_ptr(),
            line_buf.as_mut_ptr() as *mut i8,
            line_buf.len() as i32, // FIXME: panic if len is too large
            )
    };

    // shell_run diverges as by its documentation, but the wrapped signature does not show that.
    unreachable!();
}
