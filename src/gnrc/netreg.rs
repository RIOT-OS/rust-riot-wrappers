#[cfg(feature = "with_msg_v2")]
use core::mem::MaybeUninit;

use crate::error::NegativeErrorExt;

// Transmuting the pointer into a Pktsnip does the right thing by treating it as a smart
// pointer; dropping it decrements the refcount. (Otherwise we'd leak packets).
// What we drop here is what the netreg registration should consume: An owned (or lifetimed, if
// we move the handling into a closure) permission to send data to the indicated thread.
#[cfg(feature = "with_msg_v2")]
type PktsnipPort = crate::msg::v2::SendPort<
    super::pktbuf::Pktsnip<super::pktbuf::Shared>,
    { riot_sys::GNRC_NETAPI_MSG_TYPE_RCV as _ },
>;

/// Set up a netreg for a particular kind of messages
///
/// ## Roadmap
///
/// It might be convenient for this to return at some point (in case of short-lived network
/// services). Deregistration could be done and everything returned alright -- but the grant would
/// still be lost. This could be mitigated by accepting a 'static PktsnipPort or a clonable version
/// thereof -- not that anything could still be recombined after that, but at least it could be
/// used once more (with the risk that messages from the old registration arrive in the new one,
/// which is wrong correctness-wise but safe because it'll still be a pointer to a pktsnip).
#[cfg(feature = "with_msg_v2")]
pub fn register_for_messages<F: FnOnce() -> crate::Never>(
    grant: PktsnipPort,
    nettype: riot_sys::gnrc_nettype_t,
    demux_ctx: u32,
    f: F,
) -> ! {
    let mut entry = MaybeUninit::uninit();
    unsafe {
        riot_sys::gnrc_netreg_entry_init_pid(
            crate::inline_cast_mut(entry.as_mut_ptr()),
            demux_ctx,
            grant.destination().into(),
        )
    };
    let mut entry: riot_sys::gnrc_netreg_entry_t = unsafe { entry.assume_init() };

    (unsafe { riot_sys::gnrc_netreg_register(nettype, &mut entry) })
        .negative_to_error()
        .unwrap();

    f()
}
