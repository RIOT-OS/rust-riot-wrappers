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
}

impl embedded_hal::blocking::delay::DelayUs<u32> for XTimer {
    fn delay_us(&mut self, us: u32) {
        let ticks = XTimerTicks::from_micros_unchecked(us.into());
        Self::sleep(ticks);
    }
}

impl embedded_hal::blocking::delay::DelayMs<u32> for XTimer {
    fn delay_ms(&mut self, ms: u32) {
        let ticks = XTimerTicks::from_millis_unchecked(ms.into());
        Self::sleep(ticks);
    }
}

// Typically, T will be u64 or a smaller unsigned numeric type
pub struct XTimerTicks<T: Into<u64>>(T);

impl<T: Into<u64> + Copy> XTimerTicks<T> {
    fn get_low32(&self) -> u32 {
        self.0.into() as u32
    }

    fn get_high32(&self) -> u32 {
        (self.0.into() >> 32) as u32
    }
}

impl XTimerTicks<u64> {
    /// "unchecked" because no wrapping is checked for -- as long as original u32 values are fed
    /// in, that is probably OK
    fn from_micros_unchecked(micros: u64) -> Self {
        // FIXME: This is a bit like expanded static function, but without all the optimization
        // EXPANDED sys/include/xtimer/tick_conversion.h:92 (or basically the whole file)
        Self(micros * (riot_sys::XTIMER_HZ as u64) / 1000000)
    }

    fn from_millis_unchecked(millis: u64) -> Self {
        // FIXME: This is a bit like expanded static function, but without all the optimization
        // EXPANDED sys/include/xtimer/tick_conversion.h:92 (or basically the whole file)
        Self(millis * (riot_sys::XTIMER_HZ as u64) / 1000)
    }
}
