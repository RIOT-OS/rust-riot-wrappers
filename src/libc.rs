// Taken from the no_std tests of bindgen (tests/headers/no-std.h).
//
// For a more correct approach, I'd probably need to take the C compiler used for RIOT into
// account.

#![allow(non_camel_case_types)]

pub type c_int = i32;
pub type c_uint = u32;
pub type c_char = i8;
pub type c_uchar = u8;
pub type c_schar = i8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_longlong = i64;
pub type c_ulonglong = u64;

pub enum c_void {}

/// This is a limited copy of the std::ffi:c_str::CStr struct.
pub struct CStr {
    inner: [c_char],
}

fn strlen(ptr: *const c_char) -> usize
{
    let mut len = 0;
    while unsafe { ::core::slice::from_raw_parts(ptr, len + 1) }[len] != 0 {
        len = len + 1;
    }
    len
}

use ::core::str;
impl CStr {
    pub unsafe fn from_ptr<'a>(ptr: *const c_char) -> &'a CStr {
        let len = strlen(ptr);
        let ptr = ptr as *const u8;
        CStr::from_bytes_with_nul_unchecked(::core::slice::from_raw_parts(ptr, len as usize + 1))
    }

    pub unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> &CStr {
        &*(bytes as *const [u8] as *const CStr)
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.inner.as_ptr()
    }

    pub fn to_bytes_with_nul(&self) -> &[u8] {
        unsafe { &*(&self.inner as *const [c_char] as *const [u8]) }
    }

    pub fn to_bytes(&self) -> &[u8] {
        let bytes = self.to_bytes_with_nul();
        &bytes[..bytes.len() - 1]
    }

    pub fn to_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.to_bytes())
    }
}


// This is similar to the cstr-macro crate definition, but without the std dependency
#[macro_export]
macro_rules! cstr {
    ($s:expr) => (
        {
            let a = concat!($s, "\0");
            unsafe { ::riot_sys::libc::CStr::from_bytes_with_nul_unchecked(a.as_bytes()) }
        }
    )
}

#[test]
fn test()
{
    let a: &CStr = cstr!("Hello");
}
