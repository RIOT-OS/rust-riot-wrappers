use crate::stdio;
use core::fmt::Write;
use riot_sys::libc;
use riot_sys::{shell_command_t, shell_run};

// not repr(C) for as long as run() copies over all the inner commands, but there might be a time
// when we pack it into something null-terminatable from the outside and then repr(C) would help
// again. -- well actually it seems that we *need* to have an additional slot in here where to
// store the closure, and don't need to store the full struct. -- well again not right now, as
// acting on the closure would need a userdata argument which is not there (cf. freenode/#rust
// 2018-02-21 14:30CEST), so passing around callbacks directly.
#[derive(Copy, Clone)]
pub struct ShellCommand<'a, R> {
    name: &'a libc::CStr,
    desc: &'a libc::CStr,
    handler: R,
}

impl<'a, R> ShellCommand<'a, R> {
    unsafe extern "C" fn execute(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
        let commands = steal_global_run_state();
        for c in commands.iter_mut() {
            if let Some(result) = c.try_run(argc, argv) {
                return result;
            }
        }
        panic!("Command handler executed, but argv[0] does not match any known command");
    }

    pub fn new(name: &'a libc::CStr, desc: &'a libc::CStr, handler: R) -> Self {
        ShellCommand {
            name,
            desc,
            handler,
        }
    }
}

// Only implemented as a trait so there can be trait object references in the GLOBAL_RUN_STATE
pub trait ShellCommandTrait {
    fn as_shell_command(&self) -> shell_command_t;

    /// If `argv[0]` matches the command's command name, run it and return some result; otherwise
    /// do nothing and return None.
    fn try_run(&mut self, argc: libc::c_int, argv: *mut *mut libc::c_char) -> Option<libc::c_int>;
}

/// Newtype around an (argc, argv) C style string array that presents itself as much as an `&'a
/// [&'a str]` as possible. (Slicing is not implemented for reasons of laziness).
///
/// As this is used with the command line parser, it presents the individual strings as &str
/// infallibly. If non-UTF8 input is received, a variation of from_utf8_lossy is applied: The
/// complete string (rather than just the bad characters) is reported as "�", but should have the
/// same effect: Be visible as an encoding error without needlessly complicated error handling for
/// niche cases.
pub struct Args<'a>(&'a [*mut u8]);

impl<'a> Args<'a> {
    /// Create the slice from its parts.
    ///
    /// ## Unsafe
    ///
    /// argv must be a valid pointer, and its first argc items must be valid pointers. The
    /// underlying char strings do not need to be valid UTF-8, but must be null terminated.
    unsafe fn new(argc: libc::c_int, argv: *const *const libc::c_char, _lifetime_marker: &'a ()) -> Self {
        Args(core::slice::from_raw_parts(argv as _, argc as usize))
    }

    /// Returns an iterator over the arguments.
    pub fn iter(&self) -> impl Iterator<Item=&'a str> {
        let backing = self.0;
        (0..self.0.len()).map(move |i| Self::index(backing, i))
    }

    /// Helper method for indexing that does not rely on a self reference. This allows implementing
    /// iter easily; note that the iterator can live on even if the Args itself has been moved (but
    /// the 'a backing data have not).
    fn index(data: &'a [*mut u8], i: usize) -> &'a str {
        let cstr = unsafe { libc::CStr::from_ptr(data[i]) };
        core::str::from_utf8(cstr.to_bytes()).unwrap_or("�")
    }

    /// Returns the argument in the given position.
    pub fn get(&self, index: usize) -> Option<&str> {
        if index < self.0.len() {
            Some(Self::index(self.0, index))
        } else {
            None
        }
    }

    /// Length of the arguments list
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> core::ops::Index<usize> for Args<'a> {
    type Output = str;

    fn index(&self, i: usize) -> &str {
        Args::index(self.0, i)
    }
}

impl<'a, R> ShellCommandTrait for ShellCommand<'a, R>
// actually, I'd prefer to say FnMut(impl Write, &[&str]) -> i32, but impl doesn't work there.
where
    R: for<'b> FnMut(&mut stdio::Stdio, Args<'b>) -> i32,
{
    fn as_shell_command(&self) -> shell_command_t {
        shell_command_t {
            name: self.name.as_ptr(),
            desc: self.desc.as_ptr(),
            handler: Some(Self::execute),
        }
    }

    fn try_run(&mut self, argc: libc::c_int, argv: *mut *mut libc::c_char) -> Option<libc::c_int> {
        // Helper to easily be sure to create a lifetime that's only inside this function
        let marker = ();
        let args = unsafe { Args::new(argc, argv as _, &()) };

        let mut stdio = stdio::Stdio {};

        if args[0].as_bytes() == self.name.to_bytes() {
            let h = &mut self.handler;
            Some(h(&mut stdio, args))
        } else {
            None
        }
    }
}

fn null_shell_command() -> shell_command_t {
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

pub fn run(commands: &[&mut dyn ShellCommandTrait], line_buf: &mut [u8]) -> ! {
    const LIMIT: usize = 5;
    // FIXME: Arbitrary size limit, find an idiom to pass in a null-terminated slice or to allocate
    // a variable-lenth (commands.len() + 1) structure on the stack. Possibly const numeric
    // generics will solve this.
    let mut args: [shell_command_t; LIMIT + 1] = [null_shell_command(); LIMIT + 1];

    if commands.len() > LIMIT {
        panic!("Static command count exceeded");
    }

    for (src, dest) in commands.iter().zip(&mut args[..LIMIT]) {
        *dest = src.as_shell_command();
    }

    unsafe {
        if GLOBAL_RUN_STATE != 0 {
            panic!("Shell run more than once.")
        };
        GLOBAL_RUN_STATE = ::core::mem::transmute(&commands);
    }

    unsafe {
        shell_run(
            args.as_ptr() as _,
            line_buf.as_mut_ptr() as *mut _,
            line_buf.len() as i32, // FIXME: panic if len is too large
        )
    };

    // shell_run diverges as by its documentation, but the wrapped signature does not show that.
    unreachable!();
}
