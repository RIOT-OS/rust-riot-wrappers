//! This module provides a wrappers around a coap_handler::Handler in different versions, all of
//! which can be registered at a RIOT GcoapHandler.

pub mod v0_1;
pub mod v0_2;

#[deprecated(note = "Use through the v0_1 module.")]
pub use v0_1::*;
