//! # [ztimer high level timer](https://doc.riot-os.org/group__sys__ztimer.html)

#[cfg(riot_module_ztimer_periodic)]
pub mod periodic;

use core::convert::TryInto;

use riot_sys::ztimer_clock_t;

// Useful for working with durations
const NANOS_PER_SEC: u32 = 1_000_000_000;

/// A clock that knows about its frequency. The pulse length is not given in [core::time::Duration]
/// as that's not even supported by non-`min_` `const_generics`. This change, even
/// though it breaks the API.
#[derive(Copy, Clone)]
pub struct Clock<const HZ: u32>(*mut ztimer_clock_t);

#[deprecated(note = "Use the new name 'Clock' instead")]
pub type ZTimer<const HZ: u32> = Clock<HZ>;

/// A number of ticks on clocks ticking at a fixed clock speed
#[derive(Copy, Clone, Debug)]
pub struct Ticks<const HZ: u32>(pub u32);


impl<const HZ: u32> Clock<HZ> {
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
    /// It is up to the caller to select the Clock suitable for efficiency. (Even sleeping for
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
impl Clock<1> {
    /// Get the global second ZTimer clock, ZTIMER_SEC.
    ///
    /// This function is only available if the ztimer_sec module is built.
    #[cfg(riot_module_ztimer_sec)]
    #[doc(alias = "ZTIMER_SEC")]
    pub fn sec() -> Self {
        Clock(unsafe { riot_sys::ZTIMER_SEC })
    }
}

impl Clock<1000> {
    /// Get the global milliseconds ZTimer clock, ZTIMER_MSEC.
    ///
    /// This function is only available if the ztimer_msec module is built.
    #[cfg(riot_module_ztimer_msec)]
    #[doc(alias = "ZTIMER_MSEC")]
    pub fn msec() -> Self {
        Clock(unsafe { riot_sys::ZTIMER_MSEC })
    }
}

impl Clock<1000000> {
    /// Get the global microseconds ZTimer clock, ZTIMER_USEC.
    ///
    /// This function is only available if the ztimer_usec module is built.
    #[cfg(riot_module_ztimer_usec)]
    #[doc(alias = "ZTIMER_USEC")]
    pub fn usec() -> Self {
        Clock(unsafe { riot_sys::ZTIMER_USEC })
    }
}

impl embedded_hal::blocking::delay::DelayMs<u32> for Clock<1000> {
    fn delay_ms(&mut self, ms: u32) {
        self.sleep_ticks(ms.into());
    }
}

impl embedded_hal::blocking::delay::DelayUs<u32> for Clock<1000000> {
    fn delay_us(&mut self, us: u32) {
        self.sleep_ticks(us);
    }
}

/// The error type of fallible conversions to ticks.
///
/// Overflow is the only ever indicated error type; lack of accuracy in the timer does not
/// constitute a reportable error, and is always resolved by rounding up (consistent with ZTimer's
/// and Duration's behavior).
#[derive(Debug)]
pub struct Overflow;

impl<const HZ: u32> Ticks<HZ> {
    /// Maximum duration expressible on a clock with the given frequency
    const MAX: Self = Ticks(u32::MAX);

    /// Fallible conversion from a Duration
    ///
    /// This is an extra function (equivalently available as try_from) as it allows the result to
    /// be const (which many constructed durations are).
    ///
    /// Conversion is not perfect if HZ does not a divisor of $10^9$.
    ///
    /// This will be deprecated when TryFrom / TryInto can be optionally const (see
    /// <https://github.com/rust-lang/rust/issues/67792> for efforts).
    /*
    pub fn from_duration(duration: core::time::Duration) -> Result<Self, Overflow> {
        // Manual div_ceil while that's unstable, see
        // <https://github.com/rust-lang/rust/issues/88581>
        let subsec_ticks = match duration.subsec_nanos() {
            0 => 0,
            n => (n - 1) / (NANOS_PER_SEC / HZ) + 1
        };
        u32::try_from(duration.as_secs())
            .ok()
            .and_then(|s| s.checked_mul(HZ))
            .and_then(|t| t.checked_add(subsec_ticks))
            .map(|t| Ticks(t))
            .ok_or(Overflow)
    }
    */
    // Edited from the above until and_then & co are usable for const functions
    pub const fn from_duration(duration: core::time::Duration) -> Result<Self, Overflow> {
        // Manual div_ceil while that's unstable, see
        // <https://github.com/rust-lang/rust/issues/88581>
        let subsec_ticks = match duration.subsec_nanos() {
            0 => 0,
            n => (n - 1) / (NANOS_PER_SEC / HZ) + 1,
        };
        let secs = duration.as_secs();
        if secs > u32::MAX as _ {
            return Err(Overflow);
        };
        let secs = secs as u32;
        let sec_ticks = match secs.checked_mul(HZ) {
            Some(s) => s,
            _ => return Err(Overflow),
        };
        let sum_ticks = match sec_ticks.checked_add(subsec_ticks) {
            Some(s) => s,
            _ => return Err(Overflow),
        };
        Ok(Ticks(sum_ticks))
    }
}

impl<const HZ: u32> TryFrom<core::time::Duration> for Ticks<HZ> {
    type Error = Overflow;

    fn try_from(duration: core::time::Duration) -> Result<Self, Overflow> {
        Self::from_duration(duration)
    }
}
