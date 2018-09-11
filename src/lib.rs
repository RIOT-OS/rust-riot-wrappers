#![no_std]
#![feature(try_from)]
#![cfg_attr(feature = "set_panic_handler", feature(panic_handler))]
#![cfg_attr(feature = "set_panic_handler", feature(lang_items))]

extern crate riot_sys;
extern crate embedded_hal;
extern crate crc;

pub mod saul;
pub mod stdio;
pub mod thread;
pub mod shell;
pub mod i2c;
pub mod gnrc;
pub mod msg;
pub mod gcoap;

#[cfg(feature = "set_panic_handler")]
mod panic;
