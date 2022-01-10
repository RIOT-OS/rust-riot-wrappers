#[cfg(riot_module_gnrc_icmpv6)]
pub mod icmpv6;
#[cfg(riot_module_ipv6)]
pub mod ipv6;

pub mod netapi;
pub mod netreg;
pub mod pktbuf;

use riot_sys::{gnrc_netif_iter, gnrc_netif_t};

use crate::thread::KernelPID;
use core::iter::Iterator;
use core::mem::MaybeUninit;

struct NetifIter {
    current: *const gnrc_netif_t,
}

impl Iterator for NetifIter {
    type Item = *const gnrc_netif_t;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = unsafe { gnrc_netif_iter(self.current) };
        if self.current == 0 as *const gnrc_netif_t {
            None
        } else {
            Some(self.current)
        }
    }
}

/// Raw equivalent for gnrc_netif_iter; see [Netif::all] for a version that produces safely
/// usable objects.
#[doc(alias = "gnrc_netif_iter")]
pub fn netif_iter() -> impl Iterator<Item = *const gnrc_netif_t> {
    NetifIter {
        current: 0 as *const gnrc_netif_t,
    }
}

/// A registered netif
///
/// (In particular, that means that the implementation can access its fields without any further
/// synchronization).
pub struct Netif(*const gnrc_netif_t);

impl Netif {
    #[doc(alias = "gnrc_netif_iter")]
    pub fn all() -> impl Iterator<Item = Netif> {
        netif_iter().map(Netif)
    }

    #[doc(alias = "gnrc_netif_get_by_pid")]
    pub fn by_pid(pid: KernelPID) -> Option<Self> {
        const NULL: *mut riot_sys::gnrc_netif_t = 0 as _;
        // Not using as_ref: We can't guarantee that even for the short period between we're making
        // it into a reference and casting it back to a pointer again, it is not used by anyone
        // else
        match unsafe { riot_sys::gnrc_netif_get_by_pid(pid.into()) } {
            NULL => None,
            x => Some(Netif(x)),
        }
    }

    pub fn pid(&self) -> KernelPID {
        KernelPID(unsafe { (*self.0).pid })
    }

    pub fn l2addr(&self) -> &[u8] {
        unsafe { &(*self.0).l2addr[..(*self.0).l2addr_len as usize] }
    }
}

#[deprecated(note = "Use through the ipv6 module")]
pub use ipv6::*;
#[deprecated(note = "Use through the new names in ipv6")]
pub use ipv6::{
    split_address as split_ipv6_address,
    AddrList as IPv6AddrList,
    Address as IPv6Addr,
};
