use crate::{mutex, stdio};
use core::fmt::Write;
use core::any::TypeId;
use riot_sys::libc;
use riot_sys::{shell_command_t, shell_run_once};

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


/// Something that can build a suitable command array for itself and its next commands using
/// `shell_run_once` etc.
///
/// This is unsafe to impleemnt as all implementers must guarantee that a reference to the Built
/// type can be cast to a shell_command_t and that all commands in there are contiguous up until a
/// nulled one.
pub unsafe trait CommandList: HasRunCallback + Sized {
    type Built: 'static;

    fn build_shell_command<Root: HasRunCallback>(&self) -> Self::Built;

    // FIXME maybe put this into another thread so users don't have to see run_callback and
    // build_shell_command?
    fn run_once(&mut self, linebuffer: &mut [u8]) {
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
        unsafe { shell_run_once(&built as *const _ as *const _, linebuffer.as_mut_ptr(), linebuffer.len() as _) };
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

pub trait HasRunCallback {
    /// Run your own callback with argc and argv if the called argument is what the implementation
    /// put into its own entry of its Built, or defer to its next.
    fn run_callback(&mut self, command_index: TypeId, argc: i32, argv: *mut *mut u8) -> i32;
}

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
    Next: CommandList,
    H: for<'b> FnMut(&mut stdio::Stdio, Args<'b>) -> i32,
{
    name: &'a libc::CStr,
    desc: &'a libc::CStr,
    handler: H,
    next: Next,
}

impl<'a, Next, H> Command<'a, Next, H>
where
    Next: CommandList,
    H: for<'b> FnMut(&mut stdio::Stdio, Args<'b>) -> i32,
{
    // This is building a trampoline. As it's static and thus can't have the instance, we pass on
    // our own TypeId. As the length of the Next/NextBuilt tail is part of the type ID, the single root of
    // the current command group necessarily doesn't have two commands with the same id in it.
    // (One could equivalently use sizeof(Built)/sizeof(shell_command_t) and call it the negative
    // index of the command list; then we'd require Sized instead of 'static of B / Built. In the
    // inlined run_callback decision tree, that may even optimize better as it creates indices
    // usable for a jump table rather than a list of comparisons).
    extern "C" fn handle<Root: HasRunCallback>(argc: i32, argv: *mut *mut u8) -> i32 {
        let lock = CURRENT_SHELL_RUNNER
            .try_lock()
            .expect("Concurrent shell commands");
        let sleeve = lock
            .as_ref()
            .expect("Callback called while no shell set up as running");
        // unsafe: A suitable callback is always configured. We can make a &mut out of it for as
        // long as we hold the lock.
        let root = unsafe { &mut *(sleeve.0 as *mut Root) };
        let result = root.run_callback(TypeId::of::<<Self as CommandList>::Built>(), argc, argv);
        drop(root);
        drop(lock);
        result
    }
}

unsafe impl<'a, Next, H> CommandList for Command<'a, Next, H>
where
    Next: CommandList,
    H: for<'b> FnMut(&mut stdio::Stdio, Args<'b>) -> i32,
{
    type Built = BuiltCommand<Next::Built>;

    fn build_shell_command<Root: HasRunCallback>(&self) -> Self::Built {
        BuiltCommand {
            car: shell_command_t {
                name: self.name.as_ptr(),
                desc: self.desc.as_ptr(),
                handler: Some(Self::handle::<Root>),
            },
            cdr: self.next.build_shell_command::<Root>(),
        }
    }
}

impl<'a, Next, H> HasRunCallback for Command<'a, Next, H>
where
    Next: CommandList,
    H: for<'b> FnMut(&mut stdio::Stdio, Args<'b>) -> i32,
{
    // This is explicitly marked as inline as the large if / else if tree that it logically builds
    // should really be treated like a match by the optimizer, and not accumulate stack frames for
    // the commands deep down in the tree.
    #[inline]
    fn run_callback(&mut self, command_index: TypeId, argc: i32, argv: *mut *mut u8) -> i32
    {
        if command_index == TypeId::of::<<Self as CommandList>::Built>() {
            let marker = ();
            let args = unsafe { Args::new(argc, argv as _, &marker) };
            let handler = &mut self.handler;
            let mut stdio = stdio::Stdio {};
            handler(&mut stdio, args)
        } else {
            self.next.run_callback(command_index, argc, argv)
        }
    }
}

pub struct CommandListEnd;

unsafe impl CommandList for CommandListEnd {
    type Built = shell_command_t;

    fn build_shell_command<Root: HasRunCallback>(&self) -> Self::Built {
        shell_command_t {
            name: 0 as *const libc::c_char,
            desc: 0 as *const libc::c_char,
            handler: None,
        }
    }
}

impl HasRunCallback for CommandListEnd {
    fn run_callback(&mut self, _command_index: TypeId, _argc: i32, _argv: *mut *mut u8) -> i32
    {
        panic!("No handler claimed the callback");
    }
}
