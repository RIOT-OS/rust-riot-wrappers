//! Safe and idiomatic Rust wrappers for [RIOT-OS]
//!
//! See [RIOT's documentation on using Rust] for a general introduction to Rust on RIOT, [this
//! crate's README file] on general concepts (such as the interaction between modules here, RIOT
//! modules and features), and the individual modules' documentation entries for details.
//!
//! [RIOT-OS]: https://www.riot-os.org/
//! [RIOT's documentation on using Rust]: https://doc.riot-os.org/using-rust.html
//! [this crate's README file]: https://github.com/RIOT-OS/rust-riot-wrappers

#![no_std]
// for eh_personality; only needed on native
#![cfg_attr(
    all(
        feature = "set_panic_handler",
        target_arch = "x86",
        not(panic = "abort")
    ),
    feature(lang_items)
)]
// Primarily for documentation, see feature docs
#![cfg_attr(feature = "actual_never_type", feature(never_type))]
#![cfg_attr(feature = "nightly_docs", feature(fundamental))]

/// riot-sys is re-exported here as it is necessary in some places when using it to get values (eg.
/// in [error::NumericError::from_constant]). It is also used in macros such as [static_command!].
///
/// ### Stability
///
/// By directly mapping RIOT APIs, this is more volatile than the rest of riot-wrappers. Use this
/// only where necessary to utilize riot-wrappers APIs.
pub use riot_sys;

/// Re-exporting the cstr macro module because our macros in [shell] use it.
pub use cstr;

pub mod error;

mod helpers;
mod never;
use never::Never;

/// Name of the RIOT board that is being used
///
/// Development:
///
/// Once this can be const, it'll be deprecated in favor of a pub const &'static str. That'll also
/// force the compiler to remove all the exceptions at build time (currently it does not, even with
/// aggressive optimization).
pub fn board() -> &'static str {
    core::ffi::CStr::from_bytes_with_nul(riot_sys::RIOT_BOARD)
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

#[cfg(riot_module_auto_init)]
pub mod auto_init;
