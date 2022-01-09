use riot_sys::{
    gnrc_netapi_dispatch,
    gnrc_nettype_t,
    gnrc_pktbuf_release_error,
    gnrc_pktsnip_t,
    GNRC_NETAPI_MSG_TYPE_SND,
    GNRC_NETERR_SUCCESS,
};

use crate::gnrc::pktbuf::{Pktsnip, Shared};

// EXPANDED sys/include/net/gnrc/netapi.h:185
#[deprecated(note = "use riot_sys's transpiled version")]
unsafe fn gnrc_netapi_dispatch_send(
    nettype: gnrc_nettype_t,
    demux_ctx: u32,
    pkt: *mut gnrc_pktsnip_t,
) -> i32 {
    gnrc_netapi_dispatch(nettype, demux_ctx, GNRC_NETAPI_MSG_TYPE_SND as u16, pkt)
}
// EXPANDED sys/include/net/gnrc/pktbuf.h:174
#[deprecated(note = "use riot_sys's transpiled version")]
unsafe fn gnrc_pktbuf_release(pkt: *mut gnrc_pktsnip_t) {
    gnrc_pktbuf_release_error(pkt, GNRC_NETERR_SUCCESS);
}

/// Dispatch a packet to all listeners of the given nettype and demux context.
///
/// The return value indicates the number of recipients (which, unlike in the wrapped
/// gnrc_netapi_dispatch_send call, may be ignored if nondelivery of the packet is OK).
#[doc(alias = "gnrc_netapi_dispatch_send")]
pub fn dispatch_send(
    nettype: gnrc_nettype_t,
    demux_ctx: u32,
    pkt: impl Into<Pktsnip<Shared>>,
) -> i32 {
    let pkt = unsafe { pkt.into().to_ptr() };
    let subscribers = unsafe {
        riot_sys::gnrc_netapi_dispatch_send(nettype, demux_ctx, crate::inline_cast_mut(pkt))
    };
    if subscribers == 0 {
        unsafe { gnrc_pktbuf_release(pkt) };
    }
    subscribers
}
