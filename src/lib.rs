#![no_std]
#![feature(never_type)]
#![feature(const_fn)]
#![feature(const_mut_refs)]
#![feature(const_maybe_uninit_as_ptr)]
#![cfg_attr(feature = "set_panic_handler", feature(lang_items))]
#![feature(maybe_uninit_ref)]
// for embedded-nal stacks with const-sized slots, and ztimer
#![feature(min_const_generics)]
// for SAUL
#![feature(iter_map_while)]

extern crate byteorder;
extern crate crc;
extern crate embedded_hal;
extern crate riot_sys;

pub mod error;

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
#[cfg(riot_module_gnrc)]
pub mod gnrc_util;
#[cfg(riot_module_periph_i2c)]
pub mod i2c;
#[cfg(riot_module_core_msg)]
pub mod msg;

#[cfg(riot_module_periph_spi)]
pub mod spi;

#[cfg(riot_module_periph_adc)]
pub mod adc;

// Depends a lot on the XTimer internals, to the point where it breaks in combination with ZTimer.
#[cfg(all(riot_module_xtimer, not(riot_module_ztimer)))]
pub mod delay;
#[cfg(riot_module_ztimer)]
pub mod ztimer;

pub mod mutex;
#[cfg(riot_module_pthread)]
pub mod rwlock;

#[cfg(feature = "set_panic_handler")]
mod panic;

#[cfg(feature = "with_coap_handler")]
pub mod coap_handler;
#[cfg(feature = "with_coap_message")]
pub mod coap_message;

#[cfg(riot_module_sock)]
pub mod socket;
#[cfg(all(riot_module_sock, feature = "with_embedded_nal"))]
pub mod socket_embedded_nal;

#[cfg(riot_module_periph_gpio)]
pub mod gpio;

pub mod interrupt;
pub mod main;
