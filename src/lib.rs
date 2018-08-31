#![no_std]
#![feature(try_from)]
#![cfg_attr(feature = "set_panic_handler", feature(panic_handler))]
#![cfg_attr(feature = "set_panic_handler", feature(lang_items))]

extern crate embedded_hal;

pub mod libc;

pub mod raw;

pub mod saul;
pub mod stdio;
pub mod thread;
pub mod shell;
pub mod i2c;
pub mod gnrc;

#[cfg(feature = "set_panic_handler")]
mod panic;
