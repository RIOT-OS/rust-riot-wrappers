#![no_std]
#![feature(try_from)]
#![feature(never_type)]
#![feature(ptr_offset_from)]
#![cfg_attr(feature = "set_panic_handler", feature(lang_items))]

extern crate byteorder;
extern crate crc;
extern crate embedded_hal;
extern crate riot_sys;

#[cfg(riot_module_saul)]
pub mod saul;
#[cfg(riot_module_shell)]
pub mod shell;
pub mod stdio;
pub mod thread;
// internally cfg-gated as it has a no-op implementation
#[cfg(riot_module_gcoap)]
pub mod gcoap;
#[cfg(riot_module_gnrc)]
pub mod gnrc;
pub mod gnrc_util;
pub mod i2c;
#[cfg(riot_module_core_msg)]
pub mod msg;

#[cfg(riot_module_periph_spi)]
pub mod spi;

#[cfg(riot_module_periph_adc)]
pub mod adc;

pub mod mutex;
#[cfg(riot_module_pthread)]
pub mod rwlock;
pub mod delay;

#[cfg(feature = "set_panic_handler")]
mod panic;

#[cfg(feature = "with_jnet")]
extern crate jnet;
#[cfg(feature = "with_jnet")]
mod jnet_implementations;

pub mod main;
