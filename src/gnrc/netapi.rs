use riot_sys::{
    gnrc_netapi_dispatch,
    gnrc_nettype_t,
    gnrc_pktbuf_release_error,
    gnrc_pktsnip_t,
    GNRC_NETAPI_MSG_TYPE_SND,
    GNRC_NETERR_SUCCESS,
};

use gnrc::pktbuf::{Pktsnip, Shared};

// Re-implementations of the static functions
unsafe fn gnrc_netapi_dispatch_send(
    nettype: gnrc_nettype_t,
    demux_ctx: u32,
    pkt: *mut gnrc_pktsnip_t,
) -> i32 {
    gnrc_netapi_dispatch(nettype, demux_ctx, GNRC_NETAPI_MSG_TYPE_SND as u16, pkt)
}
unsafe fn gnrc_pktbuf_release(pkt: *mut gnrc_pktsnip_t) {
    gnrc_pktbuf_release_error(pkt, GNRC_NETERR_SUCCESS);
}

pub fn dispatch_send(
    nettype: gnrc_nettype_t,
    demux_ctx: u32,
    pkt: impl Into<Pktsnip<Shared>>,
) -> i32 {
    let pkt = unsafe { pkt.into().to_ptr() };
    let subscribers = unsafe { gnrc_netapi_dispatch_send(nettype, demux_ctx, pkt) };
    if subscribers == 0 {
        unsafe { gnrc_pktbuf_release(pkt) };
    }
    subscribers
}
