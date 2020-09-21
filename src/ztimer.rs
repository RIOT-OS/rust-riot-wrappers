//! # [ztimer high level timer](https://riot-os.org/api/group__sys__ztimer.html)

use riot_sys::{ztimer_clock_t};

pub struct ZTimer(*mut ztimer_clock_t);

impl ZTimer {
    /// Get the global milliseconds ZTimer, ZTIMER_MSEC.
    ///
    /// This function is only available if the ztimer_msec module is built.
    #[cfg(riot_module_ztimer_msec)]
    pub fn msec() -> Self {
        ZTimer(unsafe { riot_sys::ZTIMER_MSEC })
    }
    ///
    /// Get the global microseconds ZTimer, ZTIMER_USEC.
    ///
    /// This function is only available if the ztimer_usec module is built.
    #[cfg(riot_module_ztimer_usec)]
    pub fn usec() -> Self {
        ZTimer(unsafe { riot_sys::ZTIMER_USEC })
    }

    /// Pause the current thread for the duration of ticks in the timer's time scale.
    ///
    /// Wraps [ztimer_sleep](https://riot-os.org/api/group__sys__ztimer.html#gade98636e198f2d571c8acd861d29d360)
    pub fn sleep(&self, duration: u32) {
        unsafe { riot_sys::ztimer_sleep(self.0, duration) };
    }

    /// Keep the current thread in a busy loop until the duration of ticks in the timer's tim scale
    /// has passed
    ///
    /// Quoting the original documentation, "This blocks lower priority threads. Use only for
    /// *very* short delays.".
    ///
    /// Wraps [ztimer_spin](https://riot-os.org/api/group__sys__ztimer.html#ga9de3d9e3290746b856bb23eb2dccaa7c)
    pub fn spin(&self, duration: u32) {
        unsafe { riot_sys::ztimer_spin(self.0 as _ /* INLINE CAST */, duration) };
    }
}
