use crate::{mutex, stdio};
use core::fmt::Write;
use core::any::TypeId;
use riot_sys::libc;
use riot_sys::{shell_command_t, shell_run_once, shell_run_forever};

/// Newtype around an (argc, argv) C style string array that presents itself as much as an `&'a
/// [&'a str]` as possible. (Slicing is not implemented for reasons of laziness).
///
/// As this is used with the command line parser, it presents the individual strings as &str
/// infallibly. If non-UTF8 input is received, a variation of from_utf8_lossy is applied: The
/// complete string (rather than just the bad characters) is reported as "�", but should have the
/// same effect: Be visible as an encoding error without needlessly complicated error handling for
/// niche cases.
pub struct Args<'a>(&'a [*mut libc::c_char]);

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
    fn index(data: &'a [*mut libc::c_char], i: usize) -> &'a str {
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


/// Something that can build a suitable command array for itself and its next commands using
/// `shell_run_once` etc.
///
/// This is unsafe to impleemnt as all implementers must guarantee that a reference to the Built
/// type can be cast to a shell_command_t and that all commands in there are contiguous up until a
/// nulled one.
pub unsafe trait CommandListInternals: Sized {
    type Built: 'static;

    fn build_shell_command<Root: CommandListInternals>(&self) -> Self::Built;

    // Common code of run_once and run_forever. It is generic over F rather than taking F: unsafe
    // extern "C" because while shell_run_once is extern "C", shell_run_forever is actually in Rust
    // representation as it's a static inline in C.
    //
    // The R return value is then either () or !.
    //
    // It is set to always inline because situations in which both run_once and run_forever are
    // used are expected to be very rare.
    #[inline(always)]
    fn run_any<R, F: Fn(*const riot_sys::shell_command_t, *mut libc::c_char, i32) -> R>(
        &mut self,
        linebuffer: &mut [u8],
        cb: F
    ) -> R {
        let mut global = CURRENT_SHELL_RUNNER.lock();
        // Actually, if we really needed this, *and* could be sure that the shells are strictly
        // nested and not just started in parallel threads (how would we?), we could just stash
        // away the other callback, do our thing and revert it before leaving this function.
        assert!(global.is_none(), "Simultaneously running shells are not supported");

        let built = self.build_shell_command::<Self>();

        // mutex is maybe not the right abstraction; something different could do this if it
        // had a "put it in there until you give it back when the function returns, and users may
        // take it for some time" semantics.
        *global = Some(SleevedCommandList(self as *mut _ as *mut _));

        // Release mutex so that running shell commands can use it
        drop(global);

        // unsafe: The cast is legitimized by the convention of all Built being constructed to give
        // a null-terminated array
        let result = cb(&built as *const _ as *const riot_sys::shell_command_t, linebuffer.as_mut_ptr() as _, linebuffer.len() as _);

        CURRENT_SHELL_RUNNER.lock().take();

        result
    }

    /// Run your own callback with argc and argv if the called argument is what the implementation
    /// put into its own entry of its Built, or defer to its next.
    fn find_self_and_run(&mut self, argc: i32, argv: *mut *mut libc::c_char, command_index: TypeId) -> i32;

    #[inline(never)]
    fn find_root_and_run(argc: i32, argv: *mut *mut libc::c_char, command_index: TypeId) -> i32 {
        let lock = CURRENT_SHELL_RUNNER
            .try_lock()
            .expect("Concurrent shell commands");
        let sleeve = lock
            .as_ref()
            .expect("Callback called while no shell set up as running");
        // unsafe: A suitable callback is always configured. We can make a &mut out of it for as
        // long as we hold the lock.
        let root = unsafe { &mut *(sleeve.0 as *mut Self) };
        let result = root.find_self_and_run(argc, argv, command_index);
        drop(root);
        drop(lock);
        result
    }
}

pub trait CommandList: CommandListInternals {
    fn run_once(&mut self, linebuffer: &mut [u8]) {
        // unsafe: See unsafe in run_any where it's called
        self.run_any(linebuffer, |built, buf, len| unsafe { shell_run_once(built, buf, len) })
    }

    fn run_forever(&mut self, linebuffer: &mut [u8]) -> ! {
        // unsafe: See unsafe in run_any where it's called
        self.run_any(linebuffer, |built, buf, len| unsafe { shell_run_forever(built as _, buf, len); unreachable!() })
    }

    fn and<'a, H>(self, name: &'a libc::CStr, desc: &'a libc::CStr, handler: H) -> Command<'a, Self, H>
    where
        H: for<'b> FnMut(&mut stdio::Stdio, Args<'b>) -> i32,
    {
        Command {
            name,
            desc,
            handler,
            next: self
        }
    }
}

// For a bit more safety -- not that anything but someone stealing the module-private
// CURRENT_SHELL_RUNNER and replacing its content in an uncontrolled fashion would disturb the
// peace here --, this could be *almost* made a *mut dyn core::any::Any, and then use
// downcast_mut() in the handlers to get back the right Root, verifying in the process that indeed
// we agere on what it is in there. That currently doesn't work because the Root is not necessarily
// 'static (but typically only lives its 'a).
struct SleevedCommandList(*mut riot_sys::libc::c_void);

// unsafe: The only way we access the pointer in there is through callbacks we only let RIOT from
// the shell function, and this all happens in the same thread.
//
// (The sleeve allows putting the pointer into a global mutex in the first place).
unsafe impl Send for SleevedCommandList {}

static CURRENT_SHELL_RUNNER: mutex::Mutex<Option<SleevedCommandList>> = mutex::Mutex::new(None);

#[repr(C)]
pub struct BuiltCommand<NextBuilt> {
    car: shell_command_t,
    cdr: NextBuilt,
}

pub struct Command<'a, Next, H>
where
    Next: CommandListInternals,
    H: for<'b> FnMut(&mut stdio::Stdio, Args<'b>) -> i32,
{
    name: &'a libc::CStr,
    desc: &'a libc::CStr,
    handler: H,
    next: Next,
}

impl<'a, Next, H> Command<'a, Next, H>
where
    Next: CommandListInternals,
    H: for<'b> FnMut(&mut stdio::Stdio, Args<'b>) -> i32,
{
    // This is building a trampoline. As it's static and thus can't have the instance, we pass on
    // our own TypeId. As the length of the Next/NextBuilt tail is part of the type ID, the single root of
    // the current command group necessarily doesn't have two commands with the same id in it.
    // (One could equivalently use sizeof(Built)/sizeof(shell_command_t) and call it the negative
    // index of the command list; then we'd require Sized instead of 'static of B / Built. In the
    // inlined find_self_and_run decision tree, that may even optimize better as it creates indices
    // usable for a jump table rather than a list of comparisons).
    extern "C" fn handle<Root: CommandListInternals>(argc: i32, argv: *mut *mut libc::c_char) -> i32 {
        Root::find_root_and_run(argc, argv, TypeId::of::<<Self as CommandListInternals>::Built>())
    }
}

unsafe impl<'a, Next, H> CommandListInternals for Command<'a, Next, H>
where
    Next: CommandListInternals,
    H: for<'b> FnMut(&mut stdio::Stdio, Args<'b>) -> i32,
{
    type Built = BuiltCommand<Next::Built>;

    fn build_shell_command<Root: CommandListInternals>(&self) -> Self::Built {
        BuiltCommand {
            car: shell_command_t {
                name: self.name.as_ptr(),
                desc: self.desc.as_ptr(),
                handler: Some(Self::handle::<Root>),
            },
            cdr: self.next.build_shell_command::<Root>(),
        }
    }

    // This is explicitly marked as inline as the large if / else if tree that it logically builds
    // should really be treated like a match by the optimizer, and not accumulate stack frames for
    // the commands deep down in the tree.
    #[inline]
    fn find_self_and_run(&mut self, argc: i32, argv: *mut *mut libc::c_char, command_index: TypeId) -> i32
    {
        if command_index == TypeId::of::<<Self as CommandListInternals>::Built>() {
            let marker = ();
            let args = unsafe { Args::new(argc, argv as _, &marker) };
            let handler = &mut self.handler;
            let mut stdio = stdio::Stdio {};
            handler(&mut stdio, args)
        } else {
            self.next.find_self_and_run(argc, argv, command_index)
        }
    }
}

impl<'a, Next, H> CommandList for Command<'a, Next, H>
where
    Next: CommandListInternals,
    H: for<'b> FnMut(&mut stdio::Stdio, Args<'b>) -> i32,
{}

struct CommandListEnd;

unsafe impl CommandListInternals for CommandListEnd {
    type Built = shell_command_t;

    fn build_shell_command<Root: CommandListInternals>(&self) -> Self::Built {
        shell_command_t {
            name: 0 as *const libc::c_char,
            desc: 0 as *const libc::c_char,
            handler: None,
        }
    }

    #[inline]
    fn find_self_and_run(&mut self, _argc: i32, _argv: *mut *mut libc::c_char, _command_index: TypeId) -> i32
    {
        panic!("No handler claimed the callback");
    }
}

impl CommandList for CommandListEnd {}

pub fn new() -> impl CommandList {
    CommandListEnd
}
