pub mod netapi;
pub mod netreg;
pub mod pktbuf;

use riot_sys::{gnrc_netif_iter, gnrc_netif_t, ipv6_addr_from_str, ipv6_addr_t, kernel_pid_t};

use core::iter::Iterator;
use core::mem::MaybeUninit;
use riot_sys::libc;

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

pub fn netif_iter() -> impl Iterator<Item = *const gnrc_netif_t> {
    NetifIter {
        current: 0 as *const gnrc_netif_t,
    }
}

pub struct IPv6Addr {
    inner: ipv6_addr_t,
}

impl ::core::str::FromStr for IPv6Addr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // It'd be nice to use std::net::IPv6Addr::from_str, but the parser is generic over
        // families (maybe at some point we'll need that here too, but not now), and it's in std
        // rather then core for reasons I can't really follow.

        let s = s.as_bytes();

        let mut with_null = [0u8; 32 + 7 + 1]; // 32 nibbles + 7 colons + null byte
        if s.len() > with_null.len() - 1 {
            // Obviously too long to be a valid plain address
            return Err(());
        }
        with_null[..s.len()].copy_from_slice(s);

        let mut inner: MaybeUninit<ipv6_addr_t> = MaybeUninit::uninitialized();

        let conversion_result = unsafe {
            ipv6_addr_from_str(
                inner.as_mut_ptr(),
                libc::CStr::from_bytes_with_nul_unchecked(&with_null).as_ptr(),
            )
        };

        match conversion_result as usize {
            0 => Err(()),
            _ => Ok(Self { inner: unsafe { inner.into_initialized() } }),
        }
    }
}

impl ::core::fmt::Debug for IPv6Addr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let as_u8 = unsafe { &self.inner.u8 };
        write!(
            f,
            "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:\
             {:02x}{:02x}:{:02x}{:02x}",
            as_u8[0],
            as_u8[1],
            as_u8[2],
            as_u8[3],
            as_u8[4],
            as_u8[5],
            as_u8[6],
            as_u8[7],
            as_u8[8],
            as_u8[9],
            as_u8[10],
            as_u8[11],
            as_u8[12],
            as_u8[13],
            as_u8[14],
            as_u8[15],
        )
    }
}

impl IPv6Addr {
    pub unsafe fn as_ptr(&self) -> *const ipv6_addr_t {
        &self.inner
    }

    /// Given a ipv6_addr_t, copy the data out into an IPv6Addr.
    ///
    /// That might be inefficient in many cases, and there might be a way to get an &IPv6Addr
    /// newtyped from a &ipv6_addr_t, but right now this was simple to do.
    pub fn clone_from_ptr(raw: *const ipv6_addr_t) -> Self {
        IPv6Addr {
            inner: unsafe { *raw },
        }
    }
}

/// Given an address like fe80::1%42, split it up into a IPv6Addr and a numeric interface
/// identifier, if any is given. It is an error for the address not to be parsable, or for the
/// interface identifier not to be numeric.
///
/// Don't consider the error type final, that's just what works easily Right Now.
// This is not implemented in terms of the RIOT ipv6_addr functions as they heavily rely on
// null-terminated strings and mutating memory.
pub fn split_ipv6_address(input: &str) -> Result<(IPv6Addr, Option<kernel_pid_t>), &'static str> {
    let mut s = input.splitn(2, "%");
    let addr = s
        .next()
        .ok_or("No address")?
        .parse()
        .map_err(|_| "Unparsable address")?;
    let interface = match s.next() {
        None => None,
        Some(x) => Some(x.parse().map_err(|_| "Non-numeric interface identifier")?),
    };

    Ok((addr, interface))
}
