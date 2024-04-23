use riot_sys::gnrc_nettype_t;
use crate::gnrc::pktbuf::{Pktsnip, Shared};
use riot_sys::inline::gnrc_netapi_set;
use riot_sys::{netopt_t_NETOPT_IPV6_GROUP, size_t};
use crate::thread::KernelPID;
use core::ffi::c_void;

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
        unsafe { riot_sys::inline::gnrc_pktbuf_release(crate::inline_cast_mut(pkt)) };
    }
    subscribers
}

/// Joins an IPV6 multicast group at the provided address
pub fn join_multicast_v6(pid: KernelPID, addr: &[u8; 16]) {
    unsafe {
        let addr_ptr = addr.as_ptr() as *const c_void;
        let _ = gnrc_netapi_set(pid.into(), netopt_t_NETOPT_IPV6_GROUP, 0, addr_ptr, addr.len() as size_t);
    }
}