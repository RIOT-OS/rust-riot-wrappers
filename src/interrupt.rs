//! Interrupt helpers
//!
//! Writing interrupt handlers is something that obviously needs some care; when using this module,
//! you must understand the implications of not doing so within the CPU implementation. (Note: The
//! author does not).
//!
//! This is intended to be used for implementing special interfaces that have no generalization in
//! RIOT (eg. setting actions at particular points in a PWM cycle).

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

            // FIXME expanded from static function
            if unsafe { core::ptr::read_volatile(&riot_sys::sched_context_switch_request) } != 0 {
                unsafe { riot_sys::thread_yield_higher() };
            }
        }
    };
}
