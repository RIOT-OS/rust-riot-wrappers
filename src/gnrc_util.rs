//! Experimental area for GNRC utility functions
//!
//! These are implemented direclty in Rust and do not wrap any RIOT libraries, but seem useful at
//! least for the purpose of the author's experiments. It may turn out that they'd make nice
//! additions to the RIOT API, or are completely misguided.

use crate::gnrc::pktbuf::{Pktsnip, Shared};
#[cfg(riot_module_ipv6)]
use crate::gnrc::IPv6Addr;
use crate::thread::KernelPID;

#[cfg(riot_module_gnrc_udp)]
use riot_sys::gnrc_nettype_t_GNRC_NETTYPE_UDP as GNRC_NETTYPE_UDP;
use riot_sys::{
    gnrc_netif_hdr_t,
    gnrc_nettype_t_GNRC_NETTYPE_NETIF as GNRC_NETTYPE_NETIF,
    udp_hdr_t,
};

/// Trait of data structures that store all the information needed to respond to a Pktsnip in some
/// way; the data (typically address and port information) is copied into the trait implementation
/// and persisted there while the original snip is dropped and a new one is written.
///
/// This trait, in future, might also participate in the re-use of snips that are not dropped and
/// re-allocated but turned into responses in-place, but whether that makes sense here remains to
/// be seen.
pub trait RoundtripData {
    fn from_incoming(incoming: &Pktsnip<Shared>) -> Self;
    fn wrap(self, payload: Pktsnip<Shared>) -> Option<Pktsnip<Shared>>;
}

#[derive(Debug)]
pub struct NetifRoundtripData {
    // This would be more compact if we promised to never make UNKNOWN or ISR into KernelPIDs.
    pid: Option<KernelPID>,
}

impl RoundtripData for NetifRoundtripData {
    fn from_incoming(incoming: &Pktsnip<Shared>) -> Self {
        Self {
            pid: incoming.search_type(GNRC_NETTYPE_NETIF).map(|s| {
                let netif_hdr: &gnrc_netif_hdr_t = unsafe { &*(s.data.as_ptr() as *const _) };
                KernelPID::new(netif_hdr.if_pid).unwrap()
            }),
        }
    }

    fn wrap(self, payload: Pktsnip<Shared>) -> Option<Pktsnip<Shared>> {
        match self.pid {
            None => Some(payload),
            Some(pid) => unsafe {
                let mut netif = payload.netif_hdr_build(None, None)?;

                let data: &mut gnrc_netif_hdr_t = ::core::mem::transmute(netif.data_mut().as_ptr());
                data.if_pid = pid.into();

                Some(netif.into())
            },
        }
    }
}

#[cfg(riot_module_ipv6)]
#[derive(Debug)]
pub struct IPv6RoundtripDataFull<N: RoundtripData> {
    remote: IPv6Addr,
    local: IPv6Addr,
    // We "only" need the NetifRoundtripData if our destination address has a %interface part in it
    // -- which fortunately is a typical case during development, for otherwise that step might
    // easily be forgotten and IPv6RoundtripDataFull might be missing its next.
    next: N,
}

#[cfg(riot_module_ipv6)]
impl<N: RoundtripData> RoundtripData for IPv6RoundtripDataFull<N> {
    fn from_incoming(incoming: &Pktsnip<Shared>) -> Self {
        let ip = incoming.get_ipv6_hdr().unwrap();

        Self {
            remote: IPv6Addr::clone_from_ptr(&ip.src),
            local: IPv6Addr::clone_from_ptr(&ip.dst),
            next: N::from_incoming(incoming),
        }
    }

    fn wrap(self, payload: Pktsnip<Shared>) -> Option<Pktsnip<Shared>> {
        self.next.wrap(
            payload
                .ipv6_hdr_build(Some(&self.local), Some(&self.remote))?
                .into(),
        )
    }
}

// It'd be nice to have a UDPRoundtripData (not full) that wouldn't need to store the local port
// (b/c it's usually known in the context), but how would that get past a .wrap()-ish API?
#[cfg(riot_module_gnrc_udp)]
#[derive(Debug)]
pub struct UDPRoundtripDataFull<N: RoundtripData> {
    remote: u16,
    local: u16,
    next: N,
}

#[cfg(riot_module_gnrc_udp)]
impl<N: RoundtripData> RoundtripData for UDPRoundtripDataFull<N> {
    fn from_incoming(incoming: &Pktsnip<Shared>) -> Self {
        let (src, dst) = incoming
            .search_type(GNRC_NETTYPE_UDP)
            .map(|s| {
                let hdr: &udp_hdr_t = unsafe { &*(s.data.as_ptr() as *const _) };
                (
                    u16::from_be_bytes(unsafe { (*hdr).src_port.u8_ }),
                    u16::from_be_bytes(unsafe { (*hdr).dst_port.u8_ }),
                )
            })
            .unwrap();

        Self {
            remote: src,
            local: dst,
            next: N::from_incoming(incoming),
        }
    }

    fn wrap(self, payload: Pktsnip<Shared>) -> Option<Pktsnip<Shared>> {
        self.next
            .wrap(payload.udp_hdr_build(self.local, self.remote)?.into())
    }
}
