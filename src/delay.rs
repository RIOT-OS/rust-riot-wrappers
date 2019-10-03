use core::convert::TryInto;

/// RIOT's global XTimer, which implements a the blocking delay traits to put a thread to sleep for
/// some time.
pub struct XTimer {}

impl embedded_hal::blocking::delay::DelayUs<u32> for XTimer {
    fn delay_us(&mut self, us: u32) {
        // FIXME: This is a bit like expanded static function, but without all the optimization
        // EXPANDED sys/include/xtimer/tick_conversion.h:92 (or basically the whole file)
        let ticks = (us as u64) * (riot_sys::XTIMER_HZ as u64) / 1000000;
        let ticks = ticks.try_into().expect("Timer overflow");
        // EXPANDED sys/include/xtimer/implementation.h:166 (_xtimer_tsleep32)
        // EXPANDED sys/include/xtimer/implementation.h:180 (_xtimer_usleep)
        unsafe { riot_sys::_xtimer_tsleep(ticks, 0) };
    }
}

impl embedded_hal::blocking::delay::DelayMs<u32> for XTimer {
    fn delay_ms(&mut self, ms: u32) {
        // FIXME: This is a bit like expanded static function, but without all the optimization
        // EXPANDED sys/include/xtimer/tick_conversion.h:92 (or basically the whole file)
        let ticks = (ms as u64) * (riot_sys::XTIMER_HZ as u64) / 1000;
        let ticks = ticks.try_into().expect("Timer overflow");
        // EXPANDED sys/include/xtimer/implementation.h:166 (_xtimer_tsleep32)
        // EXPANDED sys/include/xtimer/implementation.h:180 (_xtimer_usleep)
        unsafe { riot_sys::_xtimer_tsleep(ticks, 0) };
    }
}
