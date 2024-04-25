use riot_sys::{gnrc_nettype_t, size_t};
use crate::gnrc::pktbuf::{Pktsnip, Shared};
use crate::thread::KernelPID;
use core::ffi::c_void;
use crate::gnrc::ipv6::Ipv6Addr;
use crate::error::NegativeErrorExt;

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
pub fn join_multicast_v6(scope_id: KernelPID, addr: Ipv6Addr) -> Result<(), crate::error::NumericError> {
    let octets = addr.octets();
    let addr_ptr = octets.as_ptr() as *const c_void;
    (unsafe {
        riot_sys::inline::gnrc_netapi_set(scope_id.into(), riot_sys::netopt_t_NETOPT_IPV6_GROUP, 0, addr_ptr, octets.len() as size_t)
    }).negative_to_error().map(|_| ())
}