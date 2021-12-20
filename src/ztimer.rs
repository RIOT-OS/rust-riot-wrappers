//! # [ztimer high level timer](https://doc.riot-os.org/group__sys__ztimer.html)

use core::convert::TryInto;

use riot_sys::ztimer_clock_t;

/// A ZTimer that knows about its frequency. The pulse length is not given in core::time::Duration
/// as that's not even supported by non-`min_` `const_generics`. This is likely to change, even
/// though it breaks the API.
#[derive(Copy, Clone)]
pub struct ZTimer<const HZ: u32>(*mut ztimer_clock_t);

impl<const HZ: u32> ZTimer<HZ> {
    /// Pause the current thread for the duration of ticks in the timer's time scale.
    ///
    /// Wraps [ztimer_sleep](https://doc.riot-os.org/group__sys__ztimer.html#gade98636e198f2d571c8acd861d29d360)
    #[doc(alias = "ztimer_sleep")]
    pub fn sleep_ticks(&self, duration: u32) {
        unsafe { riot_sys::ztimer_sleep(self.0, duration) };
    }

    /// Keep the current thread in a busy loop until the duration of ticks in the timer's tim scale
    /// has passed
    ///
    /// Quoting the original documentation, "This blocks lower priority threads. Use only for
    /// *very* short delays.".
    ///
    /// Wraps [ztimer_spin](https://doc.riot-os.org/group__sys__ztimer.html#ga9de3d9e3290746b856bb23eb2dccaa7c)
    #[doc(alias = "ztimer_spin")]
    pub fn spin_ticks(&self, duration: u32) {
        unsafe { riot_sys::ztimer_spin(crate::inline_cast_mut(self.0), duration) };
    }

    /// Pause the current thread for the given duration.
    ///
    /// The duration is converted into ticks (rounding up), and overflows are caught by sleeping
    /// multiple times.
    ///
    /// It is up to the caller to select the ZTimer suitable for efficiency. (Even sleeping for
    /// seconds on the microseconds timer would not overflow the timer's interface's u32, but the
    /// same multiple-sleeps trick may need to be employed by the implementation, *and* would keep
    /// the system from entering deeper sleep modes).
    pub fn sleep(&self, duration: core::time::Duration) {
        // Convert to ticks, rounding up as per Duration documentation
        let mut ticks = (duration * HZ - core::time::Duration::new(0, 1)).as_secs() + 1;
        while ticks > u32::MAX.into() {
            self.sleep_ticks(u32::MAX);
            ticks -= u64::from(u32::MAX);
        }
        self.sleep_ticks(ticks.try_into().expect("Was just checked manually above"));
    }

    /// Set the given callback to be executed in an interrupt some ticks in the future.
    ///
    /// Then, start the in_thread function from in the thread this is called from (as a regular
    /// function call).
    ///
    /// After the in_thread function terminates, the callback is dropped if it has not already
    /// triggered.
    ///
    /// Further Development:
    ///
    /// * This could probably be done with some sort of pinning instead, thus avoiding the nested
    ///   scope -- but getting the Drop right is comparatively tricky, because when done naively it
    ///   needs runtime state.
    ///
    /// * The callback could be passed something extra that enables it to set the timer again and
    ///   again. Granted, there's ztimer_periodic for these cases (and it has better drifting
    ///   properties), but for something like exponential retransmission it could be convenient.
    ///
    ///   (Might make sense to do this without an extra function variant: if the callback ignores
    ///   the timer argument and always returns None, that's all in the caller type and probebly
    ///   inlined right away).
    pub fn set_ticks_during<I: FnOnce() + Send, M: FnOnce() -> R, R>(
        &self,
        callback: I,
        ticks: u32,
        in_thread: M,
    ) -> R {
        use core::mem::ManuallyDrop;

        // This is zero-initialized, which is the more efficient mode for ztimer_t.
        let mut timer = riot_sys::ztimer_t::default();

        // FIXME: If we were worried about what this does during unwind, we might put a Drop on a
        // type around this. (But currenlty, Rust on RIOT does not unwind).
        //
        // As this is later put into timer.arg, this will need to stay put now (but we can't
        // directly Pin<&mut> it because we need ownership for the FnOnce)
        let mut callback = ManuallyDrop::new(callback);

        extern "C" fn caller<I: FnOnce() + Send>(arg: *mut riot_sys::libc::c_void) {
            // unsafe: Was cast from the same type when assigned to arg.
            let callback: &mut ManuallyDrop<I> = unsafe { &mut *(arg as *mut ManuallyDrop<I>) };
            // unsafe: The other take (actually drop) coordinates through the ztimer return value,
            // so that only one of these is ever run.
            let taken = unsafe { ManuallyDrop::take(callback) };
            taken();
        }

        timer.callback = Some(caller::<I>);
        timer.arg = &mut callback as *mut _ as *mut _;

        // unsafe: OK per C API
        unsafe {
            riot_sys::ztimer_set(self.0, &mut timer, ticks);
        }

        let result = in_thread();

        // unsafe: OK per C API
        let removed = unsafe { riot_sys::ztimer_remove(self.0, &mut timer) };

        if removed {
            // unsafe: removed == true means that the other drop (actually take) has not been run
            unsafe {
                ManuallyDrop::drop(&mut callback);
            }
        }

        result
    }
}
impl ZTimer<1> {
    /// Get the global second ZTimer, ZTIMER_SEC.
    ///
    /// This function is only available if the ztimer_sec module is built.
    #[cfg(riot_module_ztimer_sec)]
    #[doc(alias = "ZTIMER_SEC")]
    pub fn sec() -> Self {
        ZTimer(unsafe { riot_sys::ZTIMER_SEC })
    }
}

impl ZTimer<1000> {
    /// Get the global milliseconds ZTimer, ZTIMER_MSEC.
    ///
    /// This function is only available if the ztimer_msec module is built.
    #[cfg(riot_module_ztimer_msec)]
    #[doc(alias = "ZTIMER_MSEC")]
    pub fn msec() -> Self {
        ZTimer(unsafe { riot_sys::ZTIMER_MSEC })
    }
}

impl ZTimer<1000000> {
    /// Get the global microseconds ZTimer, ZTIMER_USEC.
    ///
    /// This function is only available if the ztimer_usec module is built.
    #[cfg(riot_module_ztimer_usec)]
    #[doc(alias = "ZTIMER_USEC")]
    pub fn usec() -> Self {
        ZTimer(unsafe { riot_sys::ZTIMER_USEC })
    }
}

impl embedded_hal::blocking::delay::DelayMs<u32> for ZTimer<1000> {
    fn delay_ms(&mut self, ms: u32) {
        self.sleep_ticks(ms.into());
    }
}

impl embedded_hal::blocking::delay::DelayUs<u32> for ZTimer<1000000> {
    fn delay_us(&mut self, us: u32) {
        self.sleep_ticks(us);
    }
}
