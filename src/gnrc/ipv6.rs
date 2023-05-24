//! Components for interacting with IPv6 messages on GNRC

use core::mem::MaybeUninit;

use riot_sys::{ipv6_addr_from_str, ipv6_addr_t, kernel_pid_t};

use super::pktbuf::{Mode, NotEnoughSpace, Pktsnip, Writable};
use crate::error::{NegativeErrorExt, NumericError};

impl super::Netif {
    pub fn ipv6_addrs(
        &self,
    ) -> Result<AddrList<{ riot_sys::CONFIG_GNRC_NETIF_IPV6_ADDRS_NUMOF as _ }>, NumericError> {
        let mut addrs = AddrList {
            // unsafe: as per "Initializing an array element-by-element" documentation
            addresses: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        };
        let result = unsafe {
            riot_sys::gnrc_netif_ipv6_addrs_get(
                crate::inline_cast(self.0),
                addrs.addresses.as_mut() as *mut _ as _, /* justified by array guarantees and repr(Transparent) */
                core::mem::size_of_val(&addrs.addresses) as _,
            )
        };
        addrs.len = (result.negative_to_error()? as usize) / core::mem::size_of::<Address>();
        Ok(addrs)
    }
}

/// Helper for [super::Netif::ipv6_addrs]: As the [riot_sys::gnrc_netif_ipv6_addrs_get] function requires
/// a multiple-address buffer to write in, this carries a suitable buffer.
pub struct AddrList<const MAX: usize> {
    addresses: [MaybeUninit<Address>; MAX],
    len: usize,
}

impl<const MAX: usize> core::ops::Deref for AddrList<MAX> {
    type Target = [Address];

    fn deref(&self) -> &[Address] {
        let slice = &self.addresses[..self.len];
        // unsafe: as per "Initializing an array element-by-element" documentation
        unsafe { core::mem::transmute(slice) }
    }
}

impl<'a, const MAX: usize> core::iter::IntoIterator for &'a AddrList<MAX> {
    type Item = &'a Address;

    type IntoIter = core::slice::Iter<'a, Address>;

    fn into_iter(self) -> Self::IntoIter {
        self[..].iter()
    }
}

#[repr(transparent)] // which allows the AddrList addresss to be passed to gnrc_netif_ipv6_addrs_get
#[derive(Copy, Clone)]
pub struct Address {
    inner: ipv6_addr_t,
}

// When no_std_net / embedded_nal is present, it may be a good idea to run through there (or allow
// configuration to optimize which route to take for best deduplication of code)
impl ::core::str::FromStr for Address {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // It'd be nice to use std::net::Address::from_str, but the parser is generic over
        // families (maybe at some point we'll need that here too, but not now), and it's in std
        // rather then core for reasons I can't really follow.

        let s = s.as_bytes();

        let mut with_null = [0u8; 32 + 7 + 1]; // 32 nibbles + 7 colons + null byte
        if s.len() > with_null.len() - 1 {
            // Obviously too long to be a valid plain address
            return Err(());
        }
        with_null[..s.len()].copy_from_slice(s);

        let mut inner: MaybeUninit<ipv6_addr_t> = MaybeUninit::uninit();

        let conversion_result = unsafe {
            ipv6_addr_from_str(
                inner.as_mut_ptr(),
                core::ffi::CStr::from_bytes_with_nul_unchecked(&with_null).as_ptr() as _,
            )
        };

        match conversion_result as usize {
            0 => Err(()),
            _ => Ok(Self {
                inner: unsafe { inner.assume_init() },
            }),
        }
    }
}

// When no_std_net / embedded_nal is present, it may be a good idea to run through there.
impl ::core::fmt::Debug for Address {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let as_u8 = self.raw();
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

impl Address {
    pub fn raw(&self) -> &[u8; 16] {
        unsafe { &self.inner.u8_ }
    }

    pub unsafe fn as_ptr(&self) -> *const ipv6_addr_t {
        &self.inner
    }

    /// Given a ipv6_addr_t, copy the data out into an Address.
    ///
    /// That might be inefficient in many cases, and there might be a way to get an &Address
    /// newtyped from a &ipv6_addr_t, but right now this was simple to do.
    pub fn clone_from_ptr(raw: *const ipv6_addr_t) -> Self {
        Address {
            inner: unsafe { *raw },
        }
    }

    #[doc(alias = "ipv6_addr_is_unspecified")]
    pub fn is_unspecified(&self) -> bool {
        unsafe { riot_sys::inline::ipv6_addr_is_unspecified(crate::inline_cast_ref(self)) }
    }

    #[doc(alias = "ipv6_addr_is_loopback")]
    pub fn is_loopback(&self) -> bool {
        unsafe { riot_sys::inline::ipv6_addr_is_loopback(crate::inline_cast_ref(self)) }
    }

    #[doc(alias = "ipv6_addr_is_multicast")]
    pub fn is_multicast(&self) -> bool {
        unsafe { riot_sys::inline::ipv6_addr_is_multicast(crate::inline_cast_ref(self)) }
    }

    #[doc(alias = "ipv6_addr_is_link_local")]
    pub fn is_link_local(&self) -> bool {
        unsafe { riot_sys::inline::ipv6_addr_is_link_local(crate::inline_cast_ref(self)) }
    }
}

#[cfg(feature = "with_embedded_nal")]
impl From<embedded_nal::Ipv6Addr> for Address {
    fn from(input: embedded_nal::Ipv6Addr) -> Self {
        Address {
            inner: ipv6_addr_t {
                u8_: input.octets(),
            },
        }
    }
}

#[cfg(feature = "with_embedded_nal")]
impl From<Address> for embedded_nal::Ipv6Addr {
    fn from(addr: Address) -> Self {
        Self::from(self.raw())
    }
}

/// Given an address like fe80::1%42, split it up into a Address and a numeric interface
/// identifier, if any is given. It is an error for the address not to be parsable, or for the
/// interface identifier not to be numeric.
///
/// Don't consider the error type final, that's just what works easily Right Now.
// This is not implemented in terms of the RIOT ipv6_addr functions (ipv6_addr_split_iface) as they
// heavily rely on null-terminated strings and mutating memory.
pub fn split_address(input: &str) -> Result<(Address, Option<kernel_pid_t>), &'static str> {
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

impl<M: Mode> Pktsnip<M> {
    /// Get the IPv6 header of the snip, if there is any thusly typed snip present
    // Note that we can *not* just implement this with &mut on a Writable Pktsnip, because
    // writability is only ever about the first snip
    #[doc(alias = "gnrc_ipv6_get_header")]
    pub fn ipv6_get_header(&self) -> Option<&Header> {
        // unsafe: C API, and requirement on a Pktsnip that typed snips follow that type's
        // conventions
        let ptr = unsafe { riot_sys::gnrc_ipv6_get_header(self.ptr) };
        if ptr == 0 as _ {
            None
        } else {
            // unsafe: Header is a transparent wrapper around the actual ipv6_hdr_t, and the
            // ipv6_hdr_t itself is valid as per Pktsnip reqirements
            Some(unsafe { &*(ptr as *const Header) })
        }
    }

    /// Build an IPv6 header around the Pktsnip
    #[doc(alias = "gnrc_ipv6_hdr_build")]
    pub fn ipv6_hdr_build(
        self,
        src: Option<&Address>,
        dst: Option<&Address>,
    ) -> Result<Pktsnip<Writable>, NotEnoughSpace> {
        let src = src.map(|s| unsafe { s.as_ptr() }).unwrap_or(0 as *mut _);
        let dst = dst.map(|d| unsafe { d.as_ptr() }).unwrap_or(0 as *mut _);
        let snip = unsafe { riot_sys::gnrc_ipv6_hdr_build(self.ptr, src, dst) };
        if snip == 0 as *mut _ {
            Err(NotEnoughSpace)
        } else {
            core::mem::forget(self);
            Ok(unsafe { Pktsnip::<Writable>::from_ptr(snip) })
        }
    }
}

/// A transparent wrapper around ``ipv6_hdr_t`` that provides idiomatically typed fields
#[repr(transparent)]
#[doc(alias = "ipv6_hdr_t")]
#[derive(Copy, Clone)]
pub struct Header {
    inner: riot_sys::ipv6_hdr_t,
}

impl Header {
    pub fn src(&self) -> &Address {
        // unsafe: Per transparency of the Address type
        unsafe { core::mem::transmute(&self.inner.src) }
    }

    pub fn dst(&self) -> &Address {
        // unsafe: Per transparency of the Address type
        unsafe { core::mem::transmute(&self.inner.dst) }
    }

    pub fn len(&self) -> u16 {
        // unsafe: It's a view of the fully inhabited simple union version
        u16::from_be_bytes(unsafe { self.inner.len.u8_ })
    }

    pub fn next_header(&self) -> u8 {
        self.inner.nh
    }

    pub fn hop_limit(&self) -> u8 {
        self.inner.hl
    }

    pub fn version_trafficclass_flowlabel(&self) -> &[u8; 4] {
        // unsafe: It's just a view on the network buffer we pass on unmodified
        unsafe { &self.inner.v_tc_fl.u8_ }
    }
}

impl core::fmt::Debug for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let vtcfl = self.version_trafficclass_flowlabel();
        let mut vtcfl_buf = [0u8; 8];
        hex::encode_to_slice(&vtcfl, &mut vtcfl_buf).unwrap();
        f.debug_struct("Header")
            .field(
                "version / traffic class / flow label",
                &core::str::from_utf8(&vtcfl_buf).unwrap(),
            )
            .field("len", &self.len())
            .field("next_header", &self.next_header())
            .field("hop_limit", &self.hop_limit())
            .field("src", &self.src())
            .field("dst", &self.dst())
            .finish()
    }
}
