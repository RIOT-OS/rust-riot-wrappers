//! Zero-sized types with which code in threads can safely document doing things the first time.

use core::marker::PhantomData;
use core::mem::MaybeUninit;

/// Data created for each thread that is spawned.
///
/// It encodes for permission to do anything that can only be done once per thread.
pub type StartToken = TokenParts<true, true, true>;

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
    const FLAG_SEMANTICS: bool,
    // Do we need something for "we're in a thread" factory? (Probably also doesn't need tracking
    // b/c it can be Clone -- and everything in RIOT alerady does a cheap irq_is_in check rather
    // than taking a ZST)
> {
    pub(super) _not_send: PhantomData<*const ()>,
}

impl TokenParts<true, true, true> {
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

impl<const MQ: bool, const FS: bool> TokenParts<true, MQ, FS> {
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
    ) -> (
        TokenParts<false, MQ, FS>,
        crate::msg::v2::NoConfiguredMessages,
    ) {
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

impl<const MQ: bool, const FS: bool> TokenParts<false, MQ, FS> {
    /// Inverse of [TokenParts::take_msg_semantics], indicating that the thread may be terminated again as far
    /// as message semantics are concerned.
    #[cfg(feature = "with_msg_v2")]
    pub fn return_msg_semantics(
        self,
        semantics: crate::msg::v2::NoConfiguredMessages,
    ) -> TokenParts<true, MQ, FS> {
        drop(semantics);
        TokenParts {
            _not_send: PhantomData,
        }
    }
}

impl<const MS: bool, const FS: bool> TokenParts<MS, true, FS> {
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
    pub fn with_message_queue<
        const N: usize,
        F: FnOnce(TokenParts<MS, false, FS>) -> crate::Never,
    >(
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

impl<const MQ: bool> TokenParts<true, MQ, true> {
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
