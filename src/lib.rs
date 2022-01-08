#![no_std]
#![feature(never_type)]
#![feature(const_mut_refs)]
#![cfg_attr(feature = "set_panic_handler", feature(lang_items))]
#![cfg_attr(feature = "with_coap_message", feature(generic_associated_types))]
#![feature(maybe_uninit_extra)]
// for Args IntoIterator
#![feature(type_alias_impl_trait)]
// For shell
#![feature(const_fn_trait_bound)]

pub use cstr_core as cstr;

pub mod error;

/// Name of the RIOT board that is being used
///
/// Development:
///
/// Once this can be const, it'll be deprecated in favor of a pub const &'static str. That'll also
/// force the compiler to remove all the exceptions at build time (currently it does not, even with
/// aggressive optimization).
pub fn board() -> &'static str {
    cstr::CStr::from_bytes_with_nul(riot_sys::RIOT_BOARD)
        .expect("Board names are null-terminated C strings")
        .to_str()
        .expect("Board names should be ASCII")
}

/// Cast pointers around before passing them in to functions; this is sometimes needed when a
/// struct is used from bindgen (`riot_sys::*`) but passed to a C2Rust function that uses its own
/// definition (`riot_sys::inline::*`).
///
/// Ideally this'd use what comes out of safe transmutation to statically show castability (or not
/// be needed due to better collaboration between C2Rust and bindgen), but until that's a thing,
/// checking for sizes is the least we can do.
///
/// TBD: Make this into a compile time failure (first attempts failed due to "use of generic
/// parameter from outer function" errors). Anyhow, if the check passes, the function essentially
/// becomes a no-op.
#[inline]
fn inline_cast<A, B>(input: *const A) -> *const B {
    assert_eq!(core::mem::size_of::<A>(), core::mem::size_of::<B>());
    input as _
}

/// `*mut` analogon to [inline_cast]
#[inline]
fn inline_cast_mut<A, B>(input: *mut A) -> *mut B {
    assert_eq!(core::mem::size_of::<A>(), core::mem::size_of::<B>());
    input as _
}

/// `&` analogon to [inline_cast]
#[inline]
#[allow(unused)]
unsafe fn inline_cast_ref<A, B>(input: &A) -> &B {
    assert_eq!(core::mem::size_of::<A>(), core::mem::size_of::<B>());
    core::mem::transmute(input)
}

/// `&mut` analogon to [inline_cast]
#[inline]
#[allow(unused)]
unsafe fn inline_cast_ref_mut<A, B>(input: &mut A) -> &mut B {
    assert_eq!(core::mem::size_of::<A>(), core::mem::size_of::<B>());
    core::mem::transmute(input)
}

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
#[cfg(all(riot_module_sock_udp, feature = "with_embedded_nal"))]
pub mod socket_embedded_nal;
#[cfg(all(riot_module_sock_tcp, feature = "with_embedded_nal"))]
pub mod socket_embedded_nal_tcp;

#[cfg(riot_module_periph_gpio)]
pub mod gpio;

#[cfg(riot_module_bluetil_ad)]
pub mod bluetil;

pub mod nimble {
    #[cfg(riot_module_nimble_host)]
    pub mod uuid;
}

pub mod suit;

#[cfg(riot_module_ws281x)]
pub mod ws281x;

#[cfg(riot_module_microbit)]
pub mod microbit;

#[cfg(riot_module_vfs)]
pub mod vfs;

pub mod interrupt;
#[path = "main_module.rs"]
pub mod main;

pub mod led;
