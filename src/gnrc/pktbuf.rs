use core::convert::TryInto;
use core::iter::Iterator;
use core::marker::PhantomData;
use core::mem::forget;

use riot_sys::{
    gnrc_ipv6_get_header,
    gnrc_ipv6_hdr_build,
    gnrc_netif_hdr_build,
    gnrc_nettype_t,
    gnrc_pktbuf_add,
    gnrc_pktbuf_hold,
    gnrc_pktbuf_realloc_data,
    gnrc_pktbuf_release_error,
    gnrc_pktsnip_t,
    gnrc_udp_hdr_build,
    ipv6_hdr_t,
    GNRC_NETERR_SUCCESS,
};

use crate::gnrc::IPv6Addr;

/// Error type for pktsnip operations that need free buffer space
///
/// This is not used consistently yet; some methods still return Option<P> rather than Result<P,
/// NotEnoughSpace> because they can not easily be transitioned without a breaking change.
#[derive(Debug)]
pub struct NotEnoughSpace;

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
            data: unsafe {
                ::core::slice::from_raw_parts(
                    ::core::mem::transmute(s.data),
                    s.size.try_into().unwrap(),
                )
            },
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
///
/// By constructing a Pktsnip, it is also asserted that any snip annotations are correct (for
/// example, that a `GNRC_NETTYPE_IPV6` snip does contain a full IPv6 header, as demanded by
/// `gnrc_ipv6_get_header`).
pub struct Pktsnip<M: Mode> {
    ptr: *mut gnrc_pktsnip_t,
    _phantom: PhantomData<M>,
}

/// Pktsnip can be send because any volatile fields are accessed through the appropriate functions
/// (hold, release), and the non-volatile fields are only written to by threads that made sure they
/// obtained a COW copy using start_write.
unsafe impl Send for Pktsnip<Shared> {}

/// Deprecated: Use from_ptr instead (which is unsafe)
///
/// Unfortunately, this deprecation can not use the usual annotations due to
/// https://github.com/rust-lang/rust/issues/39935
impl<M: Mode> From<*mut gnrc_pktsnip_t> for Pktsnip<M> {
    /// Accept this pointer as the refcounting wrapper's responsibility
    fn from(input: *mut gnrc_pktsnip_t) -> Self {
        Pktsnip {
            ptr: input,
            _phantom: PhantomData,
        }
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
    #[doc(alias = "gnrc_pkt_len")]
    pub fn len(&self) -> usize {
        (unsafe { riot_sys::inline::gnrc_pkt_len(crate::inline_cast(self.ptr)) }) as _
    }

    #[doc(alias = "gnrc_pkt_count")]
    pub fn count(&self) -> usize {
        (unsafe { riot_sys::inline::gnrc_pkt_count(crate::inline_cast(self.ptr)) }) as _
    }

    #[doc(alias = "gnrc_ipv6_get_header")]
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

    #[deprecated(note = "use .data() instead")]
    pub fn get_data(&self) -> &[u8] {
        self.data()
    }

    /// Return the data of only the first snip of self.
    pub fn data(&self) -> &[u8] {
        self.iter_snips().next().unwrap().data
    }

    /// Relinquish the safe Pktsnip into a pointer. The caller is responsible for calling
    /// gnrc_pktbuf_release on the result, or passing it on to someone who will.
    ///
    /// This is not *technically* unsafe as it leaks the item to get a pointer out; it's left
    /// unsafe more as a warning flag.
    pub unsafe fn to_ptr(self) -> *mut gnrc_pktsnip_t {
        let ptr = self.ptr;
        forget(self);
        ptr
    }

    /// Build a UDP header around the Pktsnip
    #[doc(alias = "gnrc_udp_hdr_build")]
    pub fn udp_hdr_build(self, src: u16, dst: u16) -> Option<Pktsnip<Writable>> {
        let snip = unsafe { gnrc_udp_hdr_build(self.ptr, src, dst) };
        if snip == 0 as *mut _ {
            None
        } else {
            forget(self);
            Some(unsafe { Pktsnip::<Writable>::from_ptr(snip) })
        }
    }

    /// Build an IPv6 header around the Pktsnip
    #[doc(alias = "gnrc_ipv6_hdr_build")]
    pub fn ipv6_hdr_build(
        self,
        src: Option<&IPv6Addr>,
        dst: Option<&IPv6Addr>,
    ) -> Option<Pktsnip<Writable>> {
        let src = src.map(|s| unsafe { s.as_ptr() }).unwrap_or(0 as *mut _);
        let dst = dst.map(|d| unsafe { d.as_ptr() }).unwrap_or(0 as *mut _);
        let snip = unsafe { gnrc_ipv6_hdr_build(self.ptr, src, dst) };
        if snip == 0 as *mut _ {
            None
        } else {
            forget(self);
            Some(unsafe { Pktsnip::<Writable>::from_ptr(snip) })
        }
    }

    // Coercing gnrc_netif_hdr_build into the same interface as udp_ and ipv6_ until I find out why
    // it's different.
    pub fn netif_hdr_build(
        self,
        src: Option<&[u8]>,
        dst: Option<&[u8]>,
    ) -> Option<Pktsnip<Writable>> {
        let (src, src_len) = src
            .map(|s| (s.as_ptr(), s.len()))
            .unwrap_or((0 as *const _, 0));
        let (dst, dst_len) = dst
            .map(|d| (d.as_ptr(), d.len()))
            .unwrap_or((0 as *const _, 0));
        // "as *mut": I'm assuming the C signature is just missing its const
        let snip = unsafe {
            gnrc_netif_hdr_build(src as *mut _, src_len as u8, dst as *mut _, dst_len as u8)
        };
        if snip == 0 as *mut _ {
            None
        } else {
            unsafe {
                (*snip).next = self.to_ptr();
                Some(Pktsnip::<Writable>::from_ptr(snip))
            }
        }
    }

    /// Allocate and prepend an uninitialized snip of given size and type to self, returning a new
    /// (writable) snip.
    pub fn add(self, size: usize, nettype: gnrc_nettype_t) -> Option<Pktsnip<Writable>> {
        Pktsnip::<Writable>::_add(Some(self), 0 as *const _, size, nettype)
    }
}

impl<'a> Pktsnip<Shared> {
    /// Take responsibility for a pointer
    ///
    /// The pointer must currently have a refcount of at least 1; dropping the result decrements
    /// it.
    pub unsafe fn from_ptr(input: *mut gnrc_pktsnip_t) -> Self {
        Pktsnip {
            ptr: input,
            _phantom: PhantomData,
        }
    }

    /// Create an exclusive version of this pktsnip
    ///
    /// This involves a copy if the current reference count is > 1.
    #[doc(alias = "gnrc_pktsnip_start_write")]
    pub fn start_write(self) -> Result<Pktsnip<Writable>, NotEnoughSpace> {
        // unsafe: The C functions justify the new type
        unsafe {
            let new = riot_sys::gnrc_pktbuf_start_write(self.to_ptr());
            if new == 0 as _ {
                Err(NotEnoughSpace)
            } else {
                Ok(Pktsnip::<Writable>::from_ptr(new))
            }
        }
    }
}

impl<'a> Pktsnip<Writable> {
    /// Take responsibility for a pointer
    ///
    /// The pointer must currently have a refcount of 1; the buffer is freed when the result is
    /// dropped.
    pub unsafe fn from_ptr(input: *mut gnrc_pktsnip_t) -> Self {
        debug_assert!(unsafe { (*input).users } == 1, "Buffer is shared");

        Pktsnip {
            ptr: input,
            _phantom: PhantomData,
        }
    }

    /// Allocate an uninitialized pktsnip. That its data is uninitialized is currently not
    /// expressed in Rust as the author thinks it's harmless (any u8 is a valid u8, and the
    /// compiler can't know that we're receiving uninitialized memory here so it can't take any
    /// shortcuts if someone ever read from it).
    pub fn allocate(size: usize, nettype: gnrc_nettype_t) -> Option<Self> {
        let next: Option<Self> = None;
        Self::_add(next, 0 as *const _, size, nettype)
    }

    /// Allocate a pktsnip and copy the slice into it.
    pub fn allocate_from(data: &[u8], nettype: gnrc_nettype_t) -> Option<Self> {
        let next: Option<Self> = None;
        Self::_add(next, data.as_ptr(), data.len(), nettype)
    }

    /// Actual wrapper around gnrc_pktbuf_add. Split into two API functions because .add() makes
    /// sense as a method, and with None as next it's more of a constructor function.
    #[doc(alias = "gnrc_pktbuf_add")]
    fn _add(
        next: Option<Pktsnip<impl Mode>>,
        data: *const u8,
        size: usize,
        nettype: gnrc_nettype_t,
    ) -> Option<Self> {
        let next = next.map(|s| s.ptr).unwrap_or(0 as *mut _);
        let snip =
            unsafe { gnrc_pktbuf_add(next, data as *const _, size.try_into().unwrap(), nettype) };
        if snip == 0 as *mut _ {
            return None;
        }
        forget(next);
        Some(unsafe { Pktsnip::<Writable>::from_ptr(snip) })
    }

    #[deprecated(note = "use data_mut")]
    pub fn get_data_mut(&'a mut self) -> &'a mut [u8] {
        self.data_mut()
    }

    pub fn data_mut(&'a mut self) -> &'a mut [u8] {
        unsafe {
            ::core::slice::from_raw_parts_mut(
                ::core::mem::transmute((*self.ptr).data),
                (*self.ptr).size.try_into().unwrap(),
            )
        }
    }

    pub fn realloc_data(&mut self, size: usize) -> Result<(), ()> {
        let result = unsafe { gnrc_pktbuf_realloc_data(self.ptr, size.try_into().unwrap()) };
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
        f.debug_struct("Pktsnip")
            .field("length", &self.len())
            .field("snips", &self.count())
            .finish()
    }
}

impl Into<Pktsnip<Shared>> for Pktsnip<Writable> {
    fn into(self) -> Pktsnip<Shared> {
        Pktsnip {
            ptr: unsafe { self.to_ptr() },
            _phantom: PhantomData,
        }
    }
}
