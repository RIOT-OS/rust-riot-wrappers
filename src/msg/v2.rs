//! A safe-to-use interface for [RIOT messages](https://doc.riot-os.org/group__core__msg.html)
//!
//! This module's main contribution is [MessageSemantics], a chaining family of ZST types that
//! represent which types a thread expects to come in on which message numbers. From that, sendable
//! [MessageAddressTicket]s can be crated that indicate to message producers that indeed the
//! indicated thread is ready to accept messages of some type on that message type.
//!
//! ## Incomplete
//!
//! Right now this can only be used for receiving messages, sending is done manually based on the
//! information in the ticket:
//!
//! ```
//! use riot_wrappers::msg::v2::MessageSemantics;
//!
//! let message_semantics = unsafe { riot_wrappers::msg::v2::NoConfiguredMessages::new() };
//! type HelloPort = riot_wrappers::msg::v2::MessagePort<(), 42>;
//! type HelloPort2 = riot_wrappers::msg::v2::MessagePort<&'static u64, 44>;
//! let (message_semantics, hello): (_, HelloPort) = message_semantics.split_off();
//! let (message_semantics, hello2): (_, HelloPort2) = message_semantics.split_off();
//!
//! println!("You may now send me messages on {:?}", hello2.ticket());
//! // You may now send me messages on MessageAddressTicket<&u64, 44> { destination: KernelPID(2) }

//!
//! static bignum: u64 = 5;
//! // Both need to live until the timer is done!
//! let mut t: riot_sys::ztimer_t = Default::default();
//! let mut m = riot_sys::msg_t {
//!     sender_pid: 99,
//!     type_: 44,
//!     content: riot_sys::msg_t__bindgen_ty_1 { ptr: &bignum as *const _ as *mut _ },
//! };
//! unsafe { riot_sys::ztimer_set_msg(riot_sys::ZTIMER_SEC, &mut t, 4, &mut m, riot_sys::thread_getpid() as _) };
//!
//! let code = message_semantics.receive()
//!     .decode(hello, |s, ()| {
//!         println!("Hello received from {:?}", s);
//!         "h"
//!     })
//!     .or_else(|m| m.decode(hello2, |s, n| {
//!         println!("Number {} received from {:?}", n, s);
//!         "n"
//!     }))
//!     .expect("Wow, an unexpected message");
//! // Number 5 received from ISR
//! println!("Result code {}", code);
//! // Result code n
//! ```

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
    // Result does not distinguish between WouldBlock and InvalidPID because the
    // MessageAddressTicket is live
    pub fn try_send(&self, data: TYPE) -> Result<(), ()> {
        let mut msg: riot_sys::msg_t = Default::default();
        msg.type_ = TYPENO;

        // See decode()
        let mut incoming = ManuallyDrop::new(data);
        core::mem::swap(&mut incoming, unsafe { core::mem::transmute(&mut msg.content) });

        let result = unsafe { riot_sys::msg_try_send(&mut msg, self.destination.into()) };
        // Outside debug, behaves like the thread isn't ready, which is quite accurate for an
        // invalid one.
        debug_assert!(result >= 0, "Target PID vanished even though a MessageAddressTicket was still around");
        match result {
            1 => Ok(()),
            _ => Err(()),
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
    fn split_off<TYPE: Send, const NEW_TYPENO: u16>(self) -> (Processing<Self, NEW_TYPENO>, MessagePort<TYPE, NEW_TYPENO>)
    {
        // Should ideally be a static assert. Checks probably happen at build time anyway due to
        // const propagation, but the panic only triggers at runtime :-(
        assert!(!self.typeno_is_known(NEW_TYPENO), "Type number is already in use for this thread.");

        // Similarly static -- better err out early
        assert!(core::mem::size_of::<TYPE>() <= core::mem::size_of::<riot_sys::msg_t__bindgen_ty_1>(), "Type is too large to be transported in a message");

        // FIXME: Same for alignment

        (
            Processing { tail: self },
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
}

pub struct Processing<TAIL: MessageSemantics, const TYPENO: u16> {
    tail: TAIL,
}

impl<TAIL: MessageSemantics, const TYPENO: u16> MessageSemantics for Processing<TAIL, TYPENO> {
    fn typeno_is_known(&self, typeno: u16) -> bool {
        if typeno == TYPENO {
            true
        } else {
            self.tail.typeno_is_known(typeno)
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
        // TBD: Let S drop it, dropping the type. While not essential for safety this does ensure
        // that droppable types are not forgotten when sent and not handled, and
        // * drop code should collapse for all trivially dropped types, and
        // * if code for a nontrivially dropped type comes from a decode() that catches it, the
        //   compiler should be able to see that b/c that value was ruled out for .type_ just
        //   before.
    }
}

impl<S: MessageSemantics> ReceivedMessage<S> {
    fn sender(&self) -> Sender {
        Sender::from_pid(self.msg.sender_pid)
    }

    pub fn decode<R, F: FnOnce(Sender, TYPE) -> R, TYPE: Send, const TYPENO: u16>(mut self, _port: &MessagePort<TYPE, TYPENO>, f: F) -> Result<R, ReceivedMessage<S>> {
        // Not actually using port, it's just the ZST on which the type constraint rides in
        if self.msg.type_ == TYPENO {
            // This'd be easier if we'd constrain transmuted to Clone...
            let mut transmuted = MaybeUninit::uninit();
            // Hoping that the compiler is clever and doesn't *really* move data around ... then
            // again, it's only 4 byte or a pointer...
            core::mem::swap(&mut transmuted, unsafe { core::mem::transmute(&mut self.msg.content) });
            let transmuted = unsafe { transmuted.assume_init() };
            Ok(f(self.sender(), transmuted))
        } else {
            Err(self)
        }
    }
}
