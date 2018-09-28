use riot_sys::{
    gnrc_ipv6_get_header,
    gnrc_netif_iter,
    gnrc_netif_t,
    gnrc_nettype_t,
    gnrc_pktbuf_add,
    gnrc_pktbuf_hold,
    gnrc_pktbuf_realloc_data,
    gnrc_pktbuf_release_error,
    gnrc_pktsnip_t,
    gnrc_ipv6_hdr_build,
    gnrc_udp_hdr_build,
    gnrc_netif_hdr_build,
    ipv6_addr_from_str,
    ipv6_addr_t,
    ipv6_hdr_t,
    kernel_pid_t,
    GNRC_NETERR_SUCCESS,
};

use core::iter::Iterator;
use riot_sys::libc;

use core::marker::PhantomData;

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

        // FIXME: use MaybeUninit when available
        let mut ret: Self = Self {
            inner: ipv6_addr_t { u8: [0; 16] },
        };

        let conversion_result = unsafe {
            ipv6_addr_from_str(
                &mut ret.inner,
                libc::CStr::from_bytes_with_nul_unchecked(&with_null).as_ptr(),
            )
        };

        match conversion_result as usize {
            0 => Err(()),
            _ => Ok(ret),
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

#[derive(Debug)]
pub struct PktsnipPart<'a> {
    pub data: &'a [u8],
    pub type_: gnrc_nettype_t,
}

pub struct SnipIter<'a> {
    pointer: *const gnrc_pktsnip_t,
    datalifetime: PhantomData<&'a ()>,
}

impl<'a> Iterator for SnipIter<'a> {
    type Item = PktsnipPart<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.pointer;
        if s == 0 as *const _ {
            return None;
        }
        let s = unsafe { *s };
        self.pointer = s.next;
        Some(PktsnipPart {
            data: unsafe { ::core::slice::from_raw_parts(::core::mem::transmute(s.data), s.size) },
            type_: s.type_,
        })
    }
}

/// Base trait for Pktsnip modes (Shared and Writable)
pub trait Mode {}

/// Marker type indicating a Pktsnip is not writable (the default in GNRC)
pub struct Shared();
impl Mode for Shared {}
/// Marker type indicating that a Pktsnip is writable by the reference owner
pub struct Writable();
impl Mode for Writable {}

/// Wrapper type around gnrc_pktsnip_t that takes care of the reference counting involved.
pub struct Pktsnip<M: Mode> {
    ptr: *mut gnrc_pktsnip_t,
    _phantom: PhantomData<M>,
}

/// Pktsnip can be send because any volatile fields are accessed through the appropriate functions
/// (hold, release), and the non-volatile fields are only written to by threads that made sure they
/// obtained a COW copy using start_write.
unsafe impl Send for Pktsnip<Shared> {}

impl<M: Mode> From<*mut gnrc_pktsnip_t> for Pktsnip<M> {
    /// Accept this pointer as the refcounting wrapper's responsibility
    // FIXME should this be unsafe?
    fn from(input: *mut gnrc_pktsnip_t) -> Self {
        Pktsnip { ptr: input, _phantom: PhantomData }
    }
}

impl Clone for Pktsnip<Shared> {
    fn clone(&self) -> Pktsnip<Shared> {
        unsafe { gnrc_pktbuf_hold(self.ptr, 1) };
        Pktsnip { ..*self }
    }
}

impl<M: Mode> Drop for Pktsnip<M> {
    fn drop(&mut self) {
        unsafe { gnrc_pktbuf_release_error(self.ptr, GNRC_NETERR_SUCCESS) }
    }
}

impl<M: Mode> Pktsnip<M> {
    pub fn len(&self) -> usize {
        // Implementing the static function gnrc_pkt_len
        self.iter_snips().map(|s| s.data.len()).sum()
    }

    pub fn count(&self) -> usize {
        // Implementing the static function gnrc_pkt_count
        self.iter_snips().count()
    }

    // Wrapper around gnrc_ipv6_get_header
    pub fn get_ipv6_hdr(&self) -> Option<&ipv6_hdr_t> {
        let hdr = unsafe { gnrc_ipv6_get_header(self.ptr) };
        if hdr == 0 as *mut _ {
            None
        } else {
            // It's OK to hand out a reference: self.ptr is immutable in its data areas, and hdr
            // should point somewhere in there
            Some(unsafe { &*hdr })
        }
    }

    pub fn iter_snips(&self) -> SnipIter {
        SnipIter {
            pointer: self.ptr,
            datalifetime: PhantomData,
        }
    }

    // This is like a wrapper around gnrc_pktsnip_search_type, but gien how simple that function
    // is, wrapping it to correct lifetimes would be more verbose than just re-implementing it.
    pub fn search_type(&self, type_: gnrc_nettype_t) -> Option<PktsnipPart> {
        self.iter_snips().filter(|x| x.type_ == type_).next()
    }

    /// Return the data of only the first snip of self.
    pub fn get_data(&self) -> &[u8] {
        self.iter_snips().next().unwrap().data
    }

    /// Relinquish the safe Pktsnip into a pointer. The caller is responsible for calling
    /// gnrc_pktbuf_release on the result, or passing it on to someone who will.
    pub unsafe fn to_ptr(self) -> *mut gnrc_pktsnip_t {
        let ptr = self.ptr;
        ::core::mem::forget(self);
        ptr
    }

    pub fn udp_hdr_build(self, src: u16, dst: u16) -> Option<Pktsnip<Writable>> {
        let snip = unsafe { gnrc_udp_hdr_build(self.to_ptr(), src, dst) };
        if snip == 0 as *mut _ {
            None
        } else {
            Some(snip.into())
        }
    }

    pub fn ipv6_hdr_build(self, src: Option<&IPv6Addr>, dst: Option<&IPv6Addr>) -> Option<Pktsnip<Writable>> {
        let src = src.map(|s| unsafe { s.as_ptr() }).unwrap_or(0 as *mut _);
        let dst = dst.map(|d| unsafe { d.as_ptr() }).unwrap_or(0 as *mut _);
        let snip = unsafe { gnrc_ipv6_hdr_build(self.to_ptr(), src, dst) };
        if snip == 0 as *mut _ {
            None
        } else {
            Some(snip.into())
        }
    }

    // Coercing gnrc_netif_hdr_build into the same interface as udp_ and ipv6_ until I find out why
    // it's different.
    pub fn netif_hdr_build(self, src: Option<&[u8]>, dst: Option<&[u8]>) -> Option<Pktsnip<Writable>> {
        let (src, src_len) = src.map(|s| (s.as_ptr(), s.len()) ).unwrap_or((0 as *const _, 0));
        let (dst, dst_len) = dst.map(|d| (d.as_ptr(), d.len()) ).unwrap_or((0 as *const _, 0));
        // "as *mut": I'm assuming the C signature is just missing its const
        let snip = unsafe { gnrc_netif_hdr_build(src as *mut _, src_len as u8, dst as *mut _, dst_len as u8) };
        if snip == 0 as *mut _ {
            None
        } else {
            unsafe { (*snip).next = self.to_ptr() };
            Some(snip.into())
        }
    }

    /// Allocate and prepend an uninitialized snip of given size and type to self, returning a new
    /// (writable) snip.
    pub fn add(self, size: usize, nettype: gnrc_nettype_t) -> Option<Pktsnip<Writable>> {
        Pktsnip::<Writable>::_add(Some(self), size, nettype)
    }
}

impl<'a> Pktsnip<Writable> {
    /// Allocate an uninitialized pktsnip. That its data is uninitialized is currently not
    /// expressed in Rust as the author thinks it's harmless (any u8 is a valid u8, and the
    /// compiler can't know that we're receiving uninitialized memory here so it can't take any
    /// shortcuts if someone ever read from it).
    pub fn allocate(size: usize, nettype: gnrc_nettype_t) -> Option<Self> {
        let next: Option<Self> = None;
        Self::_add(next, size, nettype)
    }

    /// Actual wrapper around gnrc_pktbuf_add. Split into two API functions because .add() makes
    /// sense as a method, and with None as next it's more of a constructor function.
    fn _add(next: Option<Pktsnip<impl Mode>>, size: usize, nettype: gnrc_nettype_t) -> Option<Self> {
        let next = next.map(|s| unsafe { s.to_ptr() }).unwrap_or(0 as *mut _);
        let snip = unsafe { gnrc_pktbuf_add(next, 0 as *const _, size, nettype) };
        if snip == 0 as *mut _ {
            return None;
        }
        // I *think* it's safe to not call gnrc_start_write as it's obviously my packet
        Some(snip.into())
    }

    pub fn get_data_mut(&'a mut self) -> &'a mut [u8] {
        unsafe { ::core::slice::from_raw_parts_mut(::core::mem::transmute((*self.ptr).data), (*self.ptr).size) }
    }

    pub fn realloc_data(&mut self, size: usize) -> Result<(), ()> {
        let result = unsafe { gnrc_pktbuf_realloc_data(self.ptr, size) };
        if result == 0 {
            Ok(())
        } else {
            // Actually only on ENOMEM
            Err(())
        }

    }
}

impl<M: Mode> ::core::fmt::Debug for Pktsnip<M> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(
            f,
            "Pktsnip {{ length {}, in {} snips }}",
            self.len(),
            self.count()
        )
    }
}
