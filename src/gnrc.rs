use raw::{
    gnrc_netif_iter,
    gnrc_netif_t,
    ipv6_addr_t,
    ipv6_addr_from_str,
};

use ::core::iter::Iterator;
use libc;

struct NetifIter {
    current: *const gnrc_netif_t,
}

impl Iterator for NetifIter {
    type Item = *const gnrc_netif_t;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.current = unsafe { gnrc_netif_iter(self.current) };
        if self.current == 0 as *const gnrc_netif_t {
            None
        } else {
            Some(self.current)
        }
    }
}

pub fn netif_iter() -> impl Iterator<Item = *const gnrc_netif_t> {
    NetifIter { current: 0 as *const gnrc_netif_t }
}

pub struct IPv6Addr
{
    inner: ipv6_addr_t,
}

impl ::core::str::FromStr for IPv6Addr
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // It'd be nice to use std::net::IPv6Addr::from_str, but the parser is generic over
        // families (maybe at some point we'll need that here too, but not now), and it's in std
        // rather then core for reasons I can't really follow.

        let s = s.as_bytes();

        let mut with_null = [0u8; 32 + 7 + 1]; // 32 nibbles + 7 colons + null byte
        if s.len() > with_null.len() - 1 {
            // Obviously too long to be a valid plain address
            return Err(())
        }
        with_null[..s.len()].copy_from_slice(s);

        // FIXME: use MaybeUninit when available
        let mut ret: Self = Self { inner: ipv6_addr_t { u8: [0; 16]} };

        let conversion_result = unsafe { ipv6_addr_from_str(&mut ret.inner, libc::CStr::from_bytes_with_nul_unchecked(&with_null).as_ptr()) };

        match conversion_result as usize {
            0 => Err(()),
            _ => Ok(ret),
        }
    }
}

impl ::core::fmt::Debug for IPv6Addr
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let as_u8 = unsafe { &self.inner.u8 };
        write!(f, "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:\
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

impl IPv6Addr
{
    pub unsafe fn as_ptr(&self) -> *const ipv6_addr_t {
        &self.inner
    }
}
