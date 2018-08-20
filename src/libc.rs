// Taken from the no_std tests of bindgen (tests/headers/no-std.h).
//
// For a more correct approach, I'd probably need to take the C compiler used for RIOT into
// account.

#![no_std]

pub type c_int = i32;
pub type c_char = i8;

pub enum c_void {}
