//! Interaction with interrupts
//!
//! The RIOT wrappers offer three ways to interact with interrupts:
//!
//! * Utility functions can disable interrupts (creating critical sections), check whether
//!   interrupts are enabled or to determine whether code is executed in a thread or an ISR.
//!
//! * Some functions (eg. [`ZTimer::set_ticks_during`](crate::ztimer::ZTimer::set_ticks_during))
//!   take callbacks that will be called in an interrupt context.
//!
//!   These are typechecked to be Send, as they are moved from the thread to the interrupt context.
//!
//! * Writing interrupt handlers (using the [`interrupt!`] macro).
//!
//!   Writing interrupt handlers is something that obviously needs some care; when using this module,
//!   you must understand the implications of not doing so within the CPU implementation. (Note: The
//!   author does not).
//!
//!   This is intended to be used for implementing special interfaces that have no generalization in
//!   RIOT (eg. setting actions at particular points in a PWM cycle).
//!
//! Rust code intended for use within interrupts does not generally need special precautions -- but
//! several functions (generally, anything that blocks) are discouraged (as they may fail or stall
//! the system) outside of a thread context.

/// Trivial safe wrapper for
/// [`irq_is_in`](https://doc.riot-os.org/group__core__irq.html#ga83decbeef665d955290f730125ef0e3f)
///
/// Returns true when called from an interrupt service routine
pub fn irq_is_in() -> bool {
    (unsafe { riot_sys::irq_is_in() }) != 0
}

/// Trivial safe wrapper for
/// [`irq_is_enabled`](https://doc.riot-os.org/group__core__irq.html#ga7fa965063ff2f4f4cea34f1c2a8fac25)
///
/// Returns true if interrupts are currently enabled
pub fn irq_is_enabled() -> bool {
    (unsafe { riot_sys::irq_is_enabled() }) != 0
}

/// Proof of running inside a critical section. Reexported from the [bare_metal] crate.
pub use bare_metal::CriticalSection;

/// Run a closure in the current context, but with interrupts disabled.
///
/// The function gets passed a [`bare_metal::CriticalSection`] attesting to the fact that
/// interrupts are off.
///
/// This is equivalent to the [cortex_m crate function of the same
/// name](https://docs.rs/cortex-m/latest/cortex_m/interrupt/fn.free.html).
#[doc(alias = "irq_disable")]
pub fn free<R, F: FnOnce(&CriticalSection) -> R>(f: F) -> R {
    let stored = unsafe { riot_sys::irq_disable() };

    let cs = unsafe { CriticalSection::new() };

    let ret = f(&cs);

    unsafe { riot_sys::irq_restore(stored) };
    ret
}

/// Wrap a Rust interrupt handler in an extern "C" wrapper that does the post-return cleaups.
///
/// This is probably Coretex-M specific.
///
/// The wrapped function should not panic; FIXME: Explore the use of rustig to ensure this.
#[macro_export]
macro_rules! interrupt {
    ($isr_name:ident, $rust_handler:expr) => {
        #[no_mangle]
        pub extern "C" fn $isr_name() -> () {
            $rust_handler();

            // EXPANDED cpu/cortexm_common/include/cpu.h:189 (cortexm_isr_end)
            if unsafe { core::ptr::read_volatile(&riot_sys::sched_context_switch_request) } != 0 {
                unsafe { riot_sys::thread_yield_higher() };
            }
        }
    };
}
