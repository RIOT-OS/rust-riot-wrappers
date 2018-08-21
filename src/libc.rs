// Taken from the no_std tests of bindgen (tests/headers/no-std.h).
//
// For a more correct approach, I'd probably need to take the C compiler used for RIOT into
// account.

#![no_std]

pub type c_int = i32;
pub type c_uint = u32;
pub type c_char = i8;

pub enum c_void {}

// This is not from the no_std tests but seems to be a reasonable replacement to the CStr of std

pub type CStr = [c_char]; // which only contains \0 as its mandatory last byte
// This is similar to, and adapted from, the cstr-macro crate definition, but without the std
// dependency
#[macro_export]
macro_rules! cstr {
    ($s:expr) => (
        {
            let a = concat!($s, "\0");
            let b = a.as_ref() as &'static [u8];
            let c: &'static [i8] = unsafe { ::core::mem::transmute::<&[u8], &[i8]>(b) };
            // why ::riot_sys::libc::CStr on external use but CStr for tests?
            let d = c as &'static ::riot_sys::libc::CStr;
            d
        }
    )
}

#[test]
fn test()
{
    let a: &CStr = cstr!("Hello");
}
