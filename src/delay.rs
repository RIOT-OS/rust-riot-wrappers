use core::convert::{TryInto, TryFrom};

/// RIOT's global XTimer, which implements a the blocking delay traits to put a thread to sleep for
/// some time.
pub struct XTimer {}

impl XTimer {
    pub fn sleep<T: Into<u64> + Copy>(delay: impl Into<XTimerTicks<T>>) {
        let ticks = delay.into();
        // EXPANDED sys/include/xtimer/implementation.h:166 (_xtimer_tsleep32)
        // EXPANDED sys/include/xtimer/implementation.h:180 (_xtimer_usleep)
        unsafe { riot_sys::_xtimer_tsleep(ticks.get_low32(), ticks.get_high32()) };
    }

    // FIXME consider implementing this for u32 and u64 XTimers, and only use _xtimer_set64 if
    // necessary
    pub unsafe fn set<T: Into<u64> + Copy>(cb: &mut riot_sys::xtimer_t, delay: impl Into<XTimerTicks<T>>) {
        let ticks = delay.into();
        // EXPANDED sys/include/xtimer/implementation.h:241 (xtimer_set64)
        riot_sys::_xtimer_set64(cb, ticks.get_low32(), ticks.get_high32());
    }
}

impl embedded_hal::blocking::delay::DelayUs<u32> for XTimer {
    fn delay_us(&mut self, us: u32) {
        let ticks = XTimerTicks::<u64>::from_micros_unchecked(us.into());
        Self::sleep(ticks);
    }
}

impl embedded_hal::blocking::delay::DelayMs<u32> for XTimer {
    fn delay_ms(&mut self, ms: u32) {
        let ticks = XTimerTicks::<u64>::from_millis_unchecked(ms.into());
        Self::sleep(ticks);
    }
}

// Typically, T will be u64 or a smaller unsigned numeric type
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct XTimerTicks<T: Into<u64> + Copy>(T);

impl<T: Into<u64> + Copy> XTimerTicks<T> {
    pub fn get_low32(&self) -> u32 {
        self.0.into() as u32
    }

    pub fn get_high32(&self) -> u32 {
        (self.0.into() >> 32) as u32
    }
}

// FIXME: This is a bit like expanded static function, but without all the optimization
// EXPANDED sys/include/xtimer/tick_conversion.h:92 (or basically the whole file)
impl XTimerTicks<u64> {
    /// "unchecked" because no wrapping is checked for -- as long as original u32 values are fed
    /// in, that is probably OK
    pub const fn from_micros_unchecked(micros: u64) -> Self {
        Self(micros * (riot_sys::XTIMER_HZ as u64) / 1000000)
    }

    pub const fn from_millis_unchecked(millis: u64) -> Self {
        Self(millis * (riot_sys::XTIMER_HZ as u64) / 1000)
    }

    // FIXME: may be better as a dedicated unit type with comparisons
    pub fn backoff() -> Self {
        Self(riot_sys::XTIMER_BACKOFF as u64)
    }
}

// FIXME: This is a bit like expanded static function, but without all the optimization
// EXPANDED sys/include/xtimer/tick_conversion.h:92 (or basically the whole file)
impl<T: TryFrom<u64> + Into<u64> + Copy> XTimerTicks<T> {
    pub fn try_from_micros(micros: u64) -> Option<Self> {
        Some(Self((micros.checked_mul(riot_sys::XTIMER_HZ.into())? / 1000000).try_into().ok()?))
    }

    pub fn try_from_millis(millis: u64) -> Option<Self> {
        Some(Self((millis.checked_mul(riot_sys::XTIMER_HZ.into())? / 1000).try_into().ok()?))
    }

    pub fn from_micros(micros: u64) -> Self {
        Self::try_from_micros(micros).expect("Numeric overflow")
    }

    pub fn from_millis(millis: u64) -> Self {
        Self::try_from_millis(millis).expect("Numeric overflow")
    }
}
