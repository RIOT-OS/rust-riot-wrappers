#![no_std]
#![feature(try_from)]
#![feature(ptr_offset_from)] 
#![cfg_attr(feature = "set_panic_handler", feature(lang_items))]

extern crate riot_sys;
extern crate embedded_hal;
extern crate crc;

#[cfg(riot_module_saul)]
pub mod saul;
pub mod stdio;
pub mod thread;
#[cfg(riot_module_shell)]
pub mod shell;
// internally cfg-gated as it has a no-op implementation
pub mod i2c;
#[cfg(riot_module_gnrc)]
pub mod gnrc;
#[cfg(riot_module_core_msg)]
pub mod msg;
#[cfg(riot_module_gcoap)]
pub mod gcoap;

#[cfg(feature = "set_panic_handler")]
mod panic;
