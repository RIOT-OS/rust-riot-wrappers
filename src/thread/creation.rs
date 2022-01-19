use super::{CStr, KernelPID, Status};

use core::intrinsics::transmute;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use riot_sys as raw;
use riot_sys::libc;

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
    name: &CStr,
    priority: u8,
    flags: i32,
) -> (raw::kernel_pid_t, Option<*mut riot_sys::thread_t>)
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
        Some(crate::inline_cast_mut(tcb))
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
pub fn scope<'env, F, R>(callback: F) -> R
where
    F: for<'id> FnOnce(&mut CountingThreadScope<'env, 'id>) -> R,
{
    let mut s = CountingThreadScope {
        threads: 0,
        _phantom: PhantomData,
    };

    let ret = callback(&mut s);

    s.wait_for_all();

    ret
}

/// Lifetimed helper through which threads can be spawned.
///
/// ## Lifetimes
///
/// The involved lifetimes ensure that all parts used to build the thread (its closure, stack, and
/// name) outlive the whole process, which (given the generally dynamic lifetime of threads) can
/// only be checked dynamically.
///
/// The lifetimes are:
///
/// * `'env`: A time surrounding the [`scope()`] call. All inputs to the thread are checked to live
///   at least that long (possibly longer; it is commonplace for them to be `'static`).
/// * `'id`: An identifying lifetime (or brand) of the scope. Its lifetime is somewhere inbetween
///   the outer `'env` and the run time of the called closure.
///
///   Practically, don't think of this as a lifetime, but more as a disambiguator: It makes the
///   monomorphized CountingThreadScope unique in the sense that no two instances of
///   CountingThreadScope can ever have the same type.
///
///   By having unique types, it is ensured that a counted thread is only counted down (in
///   [`.reap()`]) in the scope it was born in, and that no shenanigans with counters being swapped
///   around with [core::mem::swap()] are used to trick the compiler into allowing use-after-free.
///
/// This technique was inspired by (and is explained well) in [the GhostCell
/// Paper](http://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf).
///
pub struct CountingThreadScope<'env, 'id> {
    threads: u16, // a counter, but larger than kernel_pid_t
    _phantom: PhantomData<(&'env (), &'id ())>,
}

impl<'env, 'id> CountingThreadScope<'env, 'id> {
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
    pub fn spawn<R>(
        &mut self,
        stack: &'env mut [u8],
        closure: &'env mut R,
        name: &'env CStr,
        priority: u8,
        flags: i32,
    ) -> Result<CountedThread<'id>, raw::kernel_pid_t>
    where
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
    pub fn reap(&mut self, thread: CountedThread<'id>) {
        match thread.status() {
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

// The 'id ensures that threads can only be reaped where they were created. (It might make sense to
// move it into TrackedThread and make the tcb usable for more than just pointer comparison).
#[derive(Debug)]
pub struct CountedThread<'id> {
    thread: TrackedThread,
    _phantom: PhantomData<&'id ()>,
}

impl<'id> CountedThread<'id> {
    pub fn pid(&self) -> KernelPID {
        self.thread.pid()
    }

    pub fn status(&self) -> Status {
        self.thread.status()
    }

    #[deprecated(note = "Use .pid() instead")]
    pub fn get_pid(&self) -> KernelPID {
        self.pid()
    }

    #[deprecated(note = "Use .status() instead")]
    pub fn get_status(&self) -> Status {
        self.status()
    }
}

/// Create a thread with a statically allocated stack
pub fn spawn<R>(
    stack: &'static mut [u8],
    closure: &'static mut R,
    name: &'static CStr,
    priority: u8,
    flags: i32,
) -> Result<TrackedThread, raw::kernel_pid_t>
where
    R: Send + FnMut(),
{
    let (pid, tcb) = unsafe { create(stack, closure, name, priority, flags) };

    if pid < 0 {
        return Err(pid);
    }

    Ok(TrackedThread {
        pid: KernelPID(pid),
        tcb,
    })
}

/// A thread identified not only by its PID (which can be reused whenever the thread has quit) but
/// also by a pointer to its thread control block. This gives a TrackedThread a better get_status()
/// method that reliably reports Stopped even when the PID is reused.
///
/// A later implementation may stop actually having the pid in the struct and purely rely on the
/// tcb (although that'll need to become a lifetime'd reference to a cell by then).
#[derive(Debug)]
pub struct TrackedThread {
    pid: KernelPID,
    // If this is None, then the thread was so short-lived that the TCB couldn't even be extracted
    tcb: Option<*mut riot_sys::thread_t>,
}

impl TrackedThread {
    pub fn pid(&self) -> KernelPID {
        self.pid
    }

    /// Like status of a KernelPID, but infallible: this returnes Stopped if the PID has been
    /// re-used after our thread has stopped.
    // FIXME: This can probably be simplified a lot by just looking into the TCB if it were
    // obtained reliably
    pub fn status(&self) -> Status {
        let status = self.pid.status();
        let tcb = self.pid.thread();
        if let (Ok(status), Some(tcb), Some(startup_tcb)) = (status, tcb, self.tcb) {
            if crate::inline_cast(tcb) == startup_tcb {
                status
            } else {
                Status::Stopped
            }
        } else {
            // Thread not in task list, so it's obviousy stopped
            Status::Stopped
        }
    }

    #[deprecated(note = "Use .pid() instead")]
    pub fn get_pid(&self) -> KernelPID {
        self.pid()
    }

    #[deprecated(note = "Use .status() instead")]
    pub fn get_status(&self) -> Status {
        self.status()
    }
}

/// Data created for each thread that is spawned.
///
/// It encodes for permission to do anything that can only be done once per thread.
pub type StartToken = TokenParts<true, true>;

/// Data necessary to return from a thread that has received the [StartToken] permissions.
///
/// This is created from the initials using [TokenParts::termination()] to erase any left-over
/// information, and certifies that no actions have been taken that forbid the thread from ever
/// terminating (or if they have been taken, they have been undone).
pub struct TerminationToken {
    _not_send: PhantomData<*const ()>,
}

/// A [StartToken] that has possibly already lost some of its properties.
///
/// Note that while this item shows up in the documentation, the type is actually hidden and only
/// named as [StartToken]. This ensures that more properties can be added compatibly (because no
/// user ever names the type and would thus run into conflicts when the list of generics grows).
/// This has the downside that TokenParts can not easily be passed to downstream functions, and all
/// splitting has to happen at the top level; this should not be a problem in practice.
pub struct TokenParts<
    const MSG_SEMANTICS: bool,
    const MSG_QUEUE: bool,
    // Do we need something for "we're in a thread" factory? (Probably also doesn't need tracking
    // b/c it can be Clone -- and everything in RIOT alerady does a cheap irq_is_in check rather
    // than taking a ZST)
> {
    pub(super) _not_send: PhantomData<*const ()>,
}

impl TokenParts<true, true> {
    /// Claim that the current thread has not done anything yet that is covered by this type
    ///
    /// Do not call yourself; this needs to be public because [riot_main_with_tokens] is a macro
    /// and thus technically called from the main crate.
    pub unsafe fn new() -> Self {
        TokenParts {
            _not_send: PhantomData,
        }
    }
}

impl<const MQ: bool> TokenParts<true, MQ> {
    /// Extract the claim that the thread was not previously configured with any messages that
    /// would be sent to it.
    ///
    /// ## Example
    ///
    /// ```
    /// fn thread(tok: StartToken) -> TerminationToken {
    ///     let (tok, semantics) = tok.take_msg_semantics();
    ///     ...
    /// }
    /// ```
    #[cfg(feature = "with_msg_v2")]
    #[allow(deprecated)] // The deprecation note on NoConfiguredMessages::new only pertains to it being pub
    pub fn take_msg_semantics(
        self,
    ) -> (TokenParts<false, MQ>, crate::msg::v2::NoConfiguredMessages) {
        (
            TokenParts {
                _not_send: PhantomData,
            },
            // unsafe: This is the only safe way, and by construction running only once per thread.
            // The thread can't terminate because if it takes TokenParts it has to return a
            // termination token
            unsafe { crate::msg::v2::NoConfiguredMessages::new() },
        )
    }
}

impl<const MQ: bool> TokenParts<false, MQ> {
    /// Inverse of [TokenParts::take_msg_semantics], indicating that the thread may be terminated again as far
    /// as message semantics are concerned.
    #[cfg(feature = "with_msg_v2")]
    pub fn return_msg_semantics(
        self,
        semantics: crate::msg::v2::NoConfiguredMessages,
    ) -> TokenParts<true, MQ> {
        drop(semantics);
        TokenParts {
            _not_send: PhantomData,
        }
    }
}

impl<const MS: bool> TokenParts<MS, true> {
    /// Set up a message queue of given size N, and run the function `f` after that has been set
    /// up. `f` gets passed all the remaining thread invariants.
    ///
    /// As this doesn't deal with the message semantics, it can't be sure whether at function
    /// return time all other system components have stopped sending messages; the easy way out is
    /// to require the function to diverge.
    ///
    /// ## Example
    ///
    /// ```
    /// fn thread(tok: StartToken) -> TerminationToken {
    ///     tok.with_message_queue::<4, _>(|tok| {
    ///         ...
    ///     })
    /// }
    /// ```
    // Could also be
    //     pub fn with_message_queue<const N: usize, F: FnOnce(TokenParts<MS, false>) -> (R, crate::msg::v2::NoConfiguredMessages), R>(self, f: F) -> (R, TokenParts<true, false>) {
    // but not only would this be harder on the closure expression side, it's also slightly
    // unsound: The `true` MS would mean that the NoConfiguredMessages could be taken out again and
    // used to configure semantics, and then all of a sudden the still-configured message queue
    // would be sent to again.
    pub fn with_message_queue<const N: usize, F: FnOnce(TokenParts<MS, false>) -> !>(
        self,
        f: F,
    ) -> ! {
        assert!(
            N.count_ones() == 1,
            "Message queue sizes need to be powers of 2"
        );
        let mut queue: MaybeUninit<[riot_sys::msg_t; N]> = MaybeUninit::uninit();
        // unsafe: All requirements of the C function are satisfied
        unsafe { riot_sys::msg_init_queue(queue.as_mut_ptr() as _, N as _) };
        f(TokenParts {
            _not_send: PhantomData,
        })
    }
}

impl<const MQ: bool> TokenParts<true, MQ> {
    /// Certify that nothing has been done in this thread that precludes the termination of the
    /// thread
    ///
    /// MessageSemantics need to have been returned (or never taken) for the next thread on this
    /// PID would get weird messages.
    ///
    /// The MessageQueue is not checked for -- as all MessageSemantics have been returned (and thus
    /// nothing can safely send messages any more), even if there is a queue somewhere on the
    /// stack, it wouldn't be touched by others any more. (Of course, that's moot with the current
    /// mechanism of [TokenParts::with_message_queue()] as that diverges anyway).
    pub fn termination(self) -> TerminationToken {
        TerminationToken {
            _not_send: PhantomData,
        }
    }
}
