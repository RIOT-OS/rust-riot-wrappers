//! A safe-to-use interface for [RIOT messages](https://doc.riot-os.org/group__core__msg.html)
//!
//! This module's main contribution is [MessageSemantics], a chaining family of ZST types that
//! represent which types a thread expects to come in on which message numbers. From that, sendable
//! [MessageAddressTicket]s can be crated that indicate to message producers that indeed the
//! indicated thread is ready to accept messages of some type on that message type.
//!
//! For safety, the module relies on other components not tossing around messages indiscriminately.
//! Rust components can take a [MessageAddressTicket] and send through that (where objects'
//! lifetimes guarantee that the receiving task is still prepared to receive the message). For C
//! components, safe wrappers (TBD: will) require appropriate tickets and from there on rely on the
//! C side to only send what is described in the API.
//!
//! ## Example
//!
//! A comprehensive example of how this is currently used is maintained in [the msg_tests
//! example](https://gitlab.com/etonomy/riot-examples/-/blob/master/msg_tests/src/lib.rs).
//!
//! ## Stability
//!
//! This module is still WIP and not subject to the semver-ish conduct upheld even in pre-1.0
//! versions of riot-wrappers. The module is hidden behind the `with_msg_v2` feature to make that
//! clear.

use core::marker::PhantomData;
use core::mem::{MaybeUninit, ManuallyDrop};

use crate::thread;

/// Thread-bound information carrier that indicates that a given type number was reserved for this
/// given content type.
pub struct MessagePort<TYPE: Send, const TYPENO: u16> {
    // Can only be constructed by split_off()
    _private: (),
    // Regular PhantomData for information -- information is needd by several impl fns.
    _types: PhantomData<TYPE>,
    // Ensure MessagePort is not Send or Sync -- it is specific to the one thread it's born in
    // because it contains a statement about the current thread.
    //
    // (The alternative would be to brand the MessagePort to the thread, but that only works well
    // once there is a per-thread brand, eg. created along the mechansim described in the
    // NoConfiguredMessages::new TBD).
    _not_send: PhantomData<*const ()>,
}

impl<TYPE: Send, const TYPENO: u16> MessagePort<TYPE, TYPENO> {
    // TBD: Can't we get the promise to not quit the thread any more easily than passing lifetimes
    // with each and every ticket? Probably not. But can we get the promise in any more easily than
    // the thread unsafely stating it by upgrading its message port lifetime?
    //
    // (Plus it'll be extraordinarily tricky to do anything non-static here -- you'd need to return
    // the handed out ticket, then work the message queue to emptiness (or, if it never gets empty,
    // past the point when the ticket was returned) and only then can the port expire ... and all
    // of that needs to be encoded in the type system.
    pub fn ticket(&self) -> MessageAddressTicket<'_, TYPE, TYPENO> {
        MessageAddressTicket {
            destination: thread::get_pid(),
            _phantom: PhantomData,
        }
    }
}

pub struct MessageAddressTicket<'a, TYPE: Send, const TYPENO: u16> {
    destination: thread::KernelPID,
    _phantom: PhantomData<&'a TYPE>,
}

impl<'a, TYPE: Send, const TYPENO: u16> MessageAddressTicket<'a, TYPE, TYPENO> {
    /// Send a message to a given ticket.
    ///
    /// On success, the data is received by (or enqueued in, if a queue is set up) the thread
    /// indicated in the ticket. Otherwise, the data is returned.
    ///
    /// Note that while the underlying `msg_try_send` function knows two error cases (thread is not
    /// ready to receive, and invalid PID), the presence of a MessageAddressTicket implies that the
    /// thread promised to still be around (it may have crashed, but it can't have exited), so that
    /// error can not happen here. (If it still does due to errors in unsafe code, trips up a debug
    /// assert and else is handled like the other failure to send).
    pub fn try_send(&self, data: TYPE) -> Result<(), TYPE> {
        let mut msg: riot_sys::msg_t = Default::default();
        msg.type_ = TYPENO;

        // See extract(); this is the reverse
        let mut incoming = ManuallyDrop::new(data);
        core::mem::swap(&mut incoming, unsafe { core::mem::transmute(&mut msg.content) });

        let result = unsafe { riot_sys::msg_try_send(&mut msg, self.destination.into()) };
        // Outside debug, behaves like the thread isn't ready, which is quite accurate for an
        // invalid one.
        debug_assert!(result >= 0, "Target PID vanished even though a MessageAddressTicket was still around");
        match result {
            1 => Ok(()),
            _ => {
                // Swap back to return; the raw msg will be dropped unceremoniously.
                core::mem::swap(&mut incoming, unsafe { core::mem::transmute(&mut msg.content) });
                Err(ManuallyDrop::into_inner(incoming))
            },
        }
    }
}

impl<'a, TYPE: Send, const TYPENO: u16> core::fmt::Debug for MessageAddressTicket<'a, TYPE, TYPENO> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "MessageAddressTicket<{}, {}> {{ destination: {:?} }}", core::any::type_name::<TYPE>(), TYPENO, self.destination)
    }
}

// TBD: seal? can still unseal later.
pub trait MessageSemantics: Sized {
    // TBD: Would be great to be const
    fn typeno_is_known(&self, typeno: u16) -> bool;

    /// Reduce the type into a new MessageSemantics that knows about one more typeno, and a
    /// MessagePort that can be used to create a [MessageAddressTicket] or to process incoming
    /// messages.
    ///
    /// ```
    /// # use riot_wrappers::msg::v2::MessagePort;
    /// type NumberReceived = MessagePort<u32, 1>;
    /// type BoolReceibed = MessagePort<bool, 2>;
    /// let (message_semantics, mnum): (_, NumberReceived) = message_semantics.split_off();
    /// let (message_semantics, mbool): (_, BoolReceived) = message_semantics.split_off();
    /// ```
    ///
    /// # Panics
    ///
    /// ... if the type number has already been used, or the type is too large to be sent in a
    /// message.
    ///
    /// The conditions for these panics should be evaluatable at build time (i.e. not be part of
    /// optimized code); over time these will hopfully become static assertion errors.
    // No override should be necessary for this, not even for internal impls (see sealing above)
    fn split_off<NEW_TYPE: Send, const NEW_TYPENO: u16>(self) -> (Processing<Self, NEW_TYPE, NEW_TYPENO>, MessagePort<NEW_TYPE, NEW_TYPENO>)
    {
        // Should ideally be a static assert. Checks probably happen at build time anyway due to
        // const propagation, but the panic only triggers at runtime :-(
        assert!(!self.typeno_is_known(NEW_TYPENO), "Type number is already in use for this thread.");

        // Similarly static -- better err out early
        assert!(core::mem::size_of::<NEW_TYPE>() <= core::mem::size_of::<riot_sys::msg_t__bindgen_ty_1>(), "Type is too large to be transported in a message");

        // ... and the alignment must suffice because the data is moved in and outthrough a &mut
        // SomethingTransparent<T>
        assert!(core::mem::align_of::<NEW_TYPE>() <= core::mem::align_of::<riot_sys::msg_t__bindgen_ty_1>(), "Type has stricter alignment requirements than the message content");

        (
            Processing { tail: self, _type: PhantomData },
            MessagePort { _private: (), _types: PhantomData, _not_send: PhantomData }
        )
    }

    // No override should be necessary for this, not even for internal impls (see sealing above)
    fn receive(&self) -> ReceivedMessage<Self> {
        let mut msg = MaybeUninit::uninit();
        unsafe { riot_sys::msg_receive(msg.as_mut_ptr()) };
        let msg = unsafe { msg.assume_init() };
        ReceivedMessage {
            msg,
            _phantom: PhantomData,
        }
    }

    /// Interpret a message according to these semantics, then drop it.
    ///
    ///
    /// While not essential for safety this does ensure that droppable types are not forgotten when
    /// sent and not handled, at least if they arrive. (Can't help if someone runs try_send and
    /// does no error handling).
    ///
    /// * If all drops are trivial, this (and the [<ReceivedMessage as Drop>::drop()] caller)
    ///   should all fold into no code.
    /// * If code for a nontrivially dropped type comes after a decode(), the compiler should be
    ///   able to see that b/c that value was ruled out for .type_ just before.
    ///
    /// This is unsafe for the same reasons you can't call Drop::drop(&mut T) (the compiler forbids
    /// it).
    unsafe fn drop(message: &mut ReceivedMessage<Self>);
}

pub struct NoConfiguredMessages;

impl NoConfiguredMessages {
    /// The MessageSemantics of a thread that has made no previous commitment to receive any
    /// messages.
    ///
    /// It must only be called once in such a thread.
    ///
    /// TBD: Add a version of the thread spawner that comes with all kinds of once-per-thread
    /// gadgets.
    pub unsafe fn new() -> Self {
        Self
    }
}

impl MessageSemantics for NoConfiguredMessages {
    fn typeno_is_known(&self, _typeno: u16) -> bool {
        false
    }

    /// Panicing because if a thread receives unknown messages, it may for the same reason receive
    /// mistyped messages, and that'd be a safety violation that's better shown in the most visible
    /// way.
    ///
    /// If this is undesired, think twice about whether the source of the message really can't
    /// happen to send messages of a number this threads expects (and handles as something
    /// different) as well. If it is still undesired, you can [core::mem::forget()] the message
    /// after having decoded all desired types.
    unsafe fn drop(_message: &mut ReceivedMessage<Self>) {
        panic!("Unexpected message received");
    }
}

pub struct Processing<TAIL: MessageSemantics, TYPE, const TYPENO: u16> {
    tail: TAIL,
    // Carried around solely to be able to drop messages that did not get decoded. (Otherwise we'd
    // take solace in the fact that the MessagePort knows how to handle it, and there'll only be
    // one MessagePort of a given type in a thread, and either that one takes the message or nobody
    // does and it'd get dropped).
    _type: PhantomData<TYPE>,
}

impl<TAIL: MessageSemantics, TYPE, const TYPENO: u16> MessageSemantics for Processing<TAIL, TYPE, TYPENO> {
    fn typeno_is_known(&self, typeno: u16) -> bool {
        if typeno == TYPENO {
            true
        } else {
            self.tail.typeno_is_known(typeno)
        }
    }

    unsafe fn drop(message: &mut ReceivedMessage<Self>) {
        if message.msg.type_ == TYPENO {
            let t: TYPE = message.extract();
            drop(t);
        } else {
            TAIL::drop(core::mem::transmute(message))
        }
    }
}

pub use crate::msg::MsgSender as Sender;

#[repr(transparent)]
pub struct ReceivedMessage<S: MessageSemantics> {
    msg: riot_sys::msg_t,
    _phantom: PhantomData<S>,
}

impl<S: MessageSemantics> core::fmt::Debug for ReceivedMessage<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "ReceivedMessage {{ type: {}, sender: {:?} }}", self.msg.type_, self.sender())
    }
}

impl<S: MessageSemantics> Drop for ReceivedMessage<S> {
    #[inline]
    fn drop(&mut self) {
        unsafe { S::drop(self) };
    }
}

impl<S: MessageSemantics> ReceivedMessage<S> {
    fn sender(&self) -> Sender {
        Sender::from_pid(self.msg.sender_pid)
    }

    #[inline]
    /// Move the T out of self, leaving the msg partially uninitialized
    ///
    /// This can only be used on a type T that a MessagePort was created for.
    unsafe fn extract<T>(&mut self) -> T {
        // This'd be easier if we'd constrain transmuted to Clone...
        let mut transmuted = MaybeUninit::uninit();
        // Hoping that the compiler is clever and doesn't *really* move data around ... then
        // again, it's only 4 byte or a pointer...
        core::mem::swap(&mut transmuted, unsafe { core::mem::transmute(&mut self.msg.content) });
        unsafe { transmuted.assume_init() }
    }

    pub fn decode<R, F: FnOnce(Sender, TYPE) -> R, TYPE: Send, const TYPENO: u16>(mut self, _port: &MessagePort<TYPE, TYPENO>, f: F) -> Result<R, ReceivedMessage<S>> {
        // Not actually using the port argument, it's just the ZST on whose presence the type
        // constraint rides in. It's more for convenience of calling ("if it came on this port,
        // do...") than for correctness: The presence of a ReceivedMessage<S> instance suffices to
        // know from S's construction that TYPENO corresponds to TYPE (it can drop it too if not
        // decoded, after all).
        if self.msg.type_ == TYPENO {
            let transmuted = unsafe { self.extract() };
            let sender = self.sender();
            core::mem::forget(self); // Or else the value would be double-dropped
            Ok(f(sender, transmuted))
        } else {
            Err(self)
        }
    }
}
