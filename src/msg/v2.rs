//! A safe-to-use interface for [RIOT messages](https://doc.riot-os.org/group__core__msg.html)
//!
//! This module's main contribution is [MessageSemantics], a chaining family of ZSTs that
//! represent which types a thread expects to come in on which message numbers. From that, pairs of
//! [SendPort]s (through which other threads send messages) and [ReceivePort]s (using which
//! received messages are decoded) are split off.
//!
//! For safety, the module relies on other components not tossing around messages indiscriminately.
//! In Rust, senders are told through the SendPort how the recipient will transmute the data back.
//! For C components, safe wrappers (TBD: will) require and consume appropriate tickets and from
//! there on rely on the C side to only send what is described in the API.
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
//!
//! ## Status vs. Road map
//!
//! Conceptually, this module is built for threads that can shut down again, or for receiving
//! messages only in defined times (eg. when ). All information is preserved to allow ports split
//! off from a thread's message number pool to be recombined, pending messages to be drained and
//! the thread set up to receive completely different messages on the same message type numbers.
//!
//! Practically, this is not implemented, because threads are not commonly used in a
//! run-and-shut-down pattern in RIOT. Some documentation refers to the process of recombination
//! already.
//!
//! It might still be tricky to actually perform that recombination safely, as there can be queued
//! up messages. One pattern that is anticipated to work here is to define a single channel on
//! which a SendPort can be returned through the message queue; thus, by the time it gets returned,
//! the receiver can be sure that nothing else is left in the queue. (It might be that some API
//! changes are still needed to avoid that a [ReceivedMessage] is kept around and decoded only
//! after the port has been recombined and the number possibly reused).

use core::marker::PhantomData;
use core::mem::{ManuallyDrop, MaybeUninit};

use crate::thread;

/// Thread-bound information carrier that indicates that a given type number was reserved for this
/// given content type.
pub struct ReceivePort<TYPE: Send, const TYPENO: u16> {
    // Can only be constructed by split_off()
    _private: (),
    // Regular PhantomData for information -- information is needd by several impl fns.
    _types: PhantomData<TYPE>,
    // Ensure ReceivePort is not Send or Sync -- it is specific to the one thread it's born in
    // because it contains a statement about the current thread.
    //
    // (The alternative would be to brand the ReceivePort to the thread, but that only works well
    // once there is a per-thread brand, eg. created along the mechansim described in the
    // NoConfiguredMessages::new TBD).
    _not_send: PhantomData<*const ()>,
}

// FIXME: Maybe implement Deref to SendPort so the port can be used to send to itself?

/// Object through which messages of a precise type can be sent to a precise thread.
///
/// Unlike the ReceivePort, the SendPort is Send and Sync by addign the runtime information of the
/// destination Kernel PID to it. That process / thread is guaranteed to be live (might have
/// crashed to a non-unwinding panic but not been reused) by the construction of SendPort: A
/// SendPort can only be created when the indicated thread gives the appropriate guarantees.
///
/// It is owned, but can be used through shared references (which are Send as well); ownership
/// matters if one ever wants to stop accepting a certain type of message again.
///
/// If it is desired that multiple callers send on a single typeno (where the callers can not just
/// share a shared reference), it would be possible to create a version of the SendPort
/// that counts its clones at runtime and can only be returned when all of them are recombined, or
/// just to create a version that can be cloned at will but never recombined any more. (One way to
/// do the latter would be to add a const boolean type parameter "CLONED"; a `.clonable(self) ->
/// Self` would switch that from false to true, and then copy and clone would be implemented for
/// the result, whereas recombination would only be implemented for the CLONED = false version).
pub struct SendPort<TYPE: Send, const TYPENO: u16> {
    destination: thread::KernelPID,
    _phantom: PhantomData<TYPE>,
}

impl<TYPE: Send, const TYPENO: u16> SendPort<TYPE, TYPENO> {
    /// Send a message to a given ticket.
    ///
    /// On success, the data is received by (or enqueued in, if a queue is set up) the thread
    /// indicated in the ticket. Otherwise, the data is returned.
    ///
    /// Note that while the underlying `msg_try_send` function knows two error cases (thread is not
    /// ready to receive, and invalid PID), the presence of a SendPort implies that the
    /// thread promised to still be around (it may have crashed, but it can't have exited), so that
    /// error can not happen here. (If it still does due to errors in unsafe code, trips up a debug
    /// assert and else is handled like the other failure to send).
    pub fn try_send(&self, data: TYPE) -> Result<(), TYPE> {
        let mut msg: riot_sys::msg_t = Default::default();
        msg.type_ = TYPENO;

        // See extract(); this is the reverse
        let mut incoming = ManuallyDrop::new(data);
        core::mem::swap(&mut incoming, unsafe {
            core::mem::transmute(&mut msg.content)
        });

        let result = unsafe { riot_sys::msg_try_send(&mut msg, self.destination.into()) };
        // Outside debug, behaves like the thread isn't ready, which is quite accurate for an
        // invalid one.
        debug_assert!(
            result >= 0,
            "Target PID vanished even though a SendPort was still around"
        );
        match result {
            1 => Ok(()),
            _ => {
                // Swap back to return; the raw msg will be dropped unceremoniously.
                core::mem::swap(&mut incoming, unsafe {
                    core::mem::transmute(&mut msg.content)
                });
                Err(ManuallyDrop::into_inner(incoming))
            }
        }
    }
}

impl<TYPE: Send, const TYPENO: u16> core::fmt::Debug for ReceivePort<TYPE, TYPENO> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "ReceivePort<{}, {}> {{ }}",
            core::any::type_name::<TYPE>(),
            TYPENO,
        )
    }
}

impl<TYPE: Send, const TYPENO: u16> core::fmt::Debug for SendPort<TYPE, TYPENO> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "SendPort<{}, {}> {{ destination: {:?} }}",
            core::any::type_name::<TYPE>(),
            TYPENO,
            self.destination
        )
    }
}

/// Trait for types that indicate the current thread's readiness to receive some set of messages
///
/// In a sense, a MessageSemantics is factory for mutually nonconflicting [ReceivePort]s, and a
/// tracker of what was alerady issued.
// TBD: seal? can still unseal later.
pub trait MessageSemantics: Sized {
    // TBD: Would be great to be const
    fn typeno_is_known(&self, typeno: u16) -> bool;

    /// Reduce the type into a new MessageSemantics that knows about one more typeno, and a
    /// ReceivePort that can be used to create a [SendPort] or to process incoming
    /// messages.
    ///
    /// ```
    /// # use riot_wrappers::msg::v2::ReceivePort;
    /// type NumberReceived = ReceivePort<u32, 1>;
    /// type BoolReceibed = ReceivePort<bool, 2>;
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
    fn split_off<NEW_TYPE: Send, const NEW_TYPENO: u16>(
        self,
    ) -> (
        Processing<Self, NEW_TYPE, NEW_TYPENO>,
        ReceivePort<NEW_TYPE, NEW_TYPENO>,
        SendPort<NEW_TYPE, NEW_TYPENO>,
    ) {
        // Should ideally be a static assert. Checks probably happen at build time anyway due to
        // const propagation, but the panic only triggers at runtime :-(
        assert!(
            !self.typeno_is_known(NEW_TYPENO),
            "Type number is already in use for this thread."
        );

        // Similarly static -- better err out early
        assert!(
            core::mem::size_of::<NEW_TYPE>()
                <= core::mem::size_of::<riot_sys::msg_t__bindgen_ty_1>(),
            "Type is too large to be transported in a message"
        );

        // ... and the alignment must suffice because the data is moved in and outthrough a &mut
        // SomethingTransparent<T>
        assert!(
            core::mem::align_of::<NEW_TYPE>()
                <= core::mem::align_of::<riot_sys::msg_t__bindgen_ty_1>(),
            "Type has stricter alignment requirements than the message content"
        );

        (
            Processing {
                tail: self,
                _type: PhantomData,
            },
            ReceivePort {
                _private: (),
                _types: PhantomData,
                _not_send: PhantomData,
            },
            SendPort {
                destination: thread::get_pid(),
                _phantom: PhantomData,
            },
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

/// The MessageSemantics of a thread that has made no previous commitment to receive any
/// messages.
impl NoConfiguredMessages {
    /// Create a new MessageSemantics object to split into [ReceivePort]s.
    ///
    /// **Conditions**, violating which is a safety violation:
    ///
    /// * The thread must currently not allow sending any messages to it, or even created an
    ///   otherwise unused NoConfiguredMessages
    ///
    /// * The thread must not terminate.
    ///
    /// TBD: Add a version of the thread spawner that comes with all kinds of once-per-thread
    /// gadgets.
    pub unsafe fn new() -> Self {
        Self
    }

    /// Create a new MessageSemantics object to split into [ReceivePort]s in a scope.
    ///
    /// This is somewhat safer to use than [new()] because by taking the NoConfiguredMessages
    /// object back (which currently can only be done by not splitting off anything, and later by
    /// returning everything that was split off); an easy way to do that is to just never return.
    ///
    /// **Conditions**, violating which is a safety violation:
    ///
    /// * The thread must currently not allow sending any messages to it, or even created an
    ///   otherwise unused NoConfiguredMessages
    pub unsafe fn new_scoped(f: impl FnOnce(Self) -> Self) {
        f(Self);
        // FIXME: ensure that the queue is flushed (eg. by sending a terminal message, or by
        // asserting that the queue is empty)
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
    // take solace in the fact that the ReceivePort knows how to handle it, and there'll only be
    // one ReceivePort of a given type in a thread, and either that one takes the message or nobody
    // does and it'd get dropped).
    _type: PhantomData<TYPE>,
}

impl<TAIL: MessageSemantics, TYPE, const TYPENO: u16> MessageSemantics
    for Processing<TAIL, TYPE, TYPENO>
{
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
        write!(
            f,
            "ReceivedMessage {{ type: {}, sender: {:?} }}",
            self.msg.type_,
            self.sender()
        )
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
    /// This can only be used on a type T that a ReceivePort was created for.
    unsafe fn extract<T>(&mut self) -> T {
        // This'd be easier if we'd constrain transmuted to Clone...
        let mut transmuted = MaybeUninit::uninit();
        // Hoping that the compiler is clever and doesn't *really* move data around ... then
        // again, it's only 4 byte or a pointer...
        core::mem::swap(&mut transmuted, unsafe {
            core::mem::transmute(&mut self.msg.content)
        });
        unsafe { transmuted.assume_init() }
    }

    pub fn decode<R, F: FnOnce(Sender, TYPE) -> R, TYPE: Send, const TYPENO: u16>(
        mut self,
        _port: &ReceivePort<TYPE, TYPENO>,
        f: F,
    ) -> Result<R, ReceivedMessage<S>> {
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
