//! Wrappers for the `LEDn_{ON,OFF,TOGGLE}` macros

use core::convert::Infallible;

/// The Ith LED (calling the `LED<I>_{ON,OFF,TOGGLE}` macros).
///
/// The preferred interface for turning a LED on and off is [switch_hal::OutputSwitch].
///
/// LEDs are accessible safely; any not implemented on a board are silently ignored.
///
/// LEDs are wrapped into embedded-hal 0.2 GPIOs for compatibility reasons (but that integration is
/// discontinued with embedded-hal 1.0); GPIO is interpreted such that "high" is having the LED on,
/// and "low" is off.
pub struct LED<const I: u8>(());

impl<const I: u8> LED<I> {
    pub const fn new() -> Self {
        assert!(I < 8, "RIOT only defines LED0..7");
        Self(())
    }
}

impl<const I: u8> switch_hal::OutputSwitch for LED<I> {
    type Error = Infallible;

    fn on(&mut self) -> Result<(), Self::Error> {
        use embedded_hal_0_2::digital::v2::OutputPin;
        self.set_high()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        use embedded_hal_0_2::digital::v2::OutputPin;
        self.set_low()
    }
}

impl<const I: u8> switch_hal::ToggleableOutputSwitch for LED<I> {
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        <Self as embedded_hal_0_2::digital::v2::ToggleableOutputPin>::toggle(self)
    }
}

impl<const I: u8> embedded_hal_0_2::digital::v2::OutputPin for LED<I> {
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Infallible> {
        // unsafe: RIOT's LED functions can be called any time (and no-op on undefined LEDs)
        unsafe {
            match I {
                0 => riot_sys::macro_LED0_ON(),
                1 => riot_sys::macro_LED1_ON(),
                2 => riot_sys::macro_LED2_ON(),
                3 => riot_sys::macro_LED3_ON(),
                4 => riot_sys::macro_LED4_ON(),
                5 => riot_sys::macro_LED5_ON(),
                6 => riot_sys::macro_LED6_ON(),
                7 => riot_sys::macro_LED7_ON(),
                _ => unreachable!(),
            }
        };
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Infallible> {
        // unsafe: RIOT's LED functions can be called any time (and no-op on undefined LEDs)
        unsafe {
            match I {
                0 => riot_sys::macro_LED0_OFF(),
                1 => riot_sys::macro_LED1_OFF(),
                2 => riot_sys::macro_LED2_OFF(),
                3 => riot_sys::macro_LED3_OFF(),
                4 => riot_sys::macro_LED4_OFF(),
                5 => riot_sys::macro_LED5_OFF(),
                6 => riot_sys::macro_LED6_OFF(),
                7 => riot_sys::macro_LED7_OFF(),
                _ => unreachable!(),
            }
        };
        Ok(())
    }
}

impl<const I: u8> embedded_hal_0_2::digital::v2::ToggleableOutputPin for LED<I> {
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Infallible> {
        // unsafe: RIOT's LED functions can be called any time (and no-op on undefined LEDs)
        unsafe {
            match I {
                0 => riot_sys::macro_LED0_TOGGLE(),
                1 => riot_sys::macro_LED1_TOGGLE(),
                2 => riot_sys::macro_LED2_TOGGLE(),
                3 => riot_sys::macro_LED3_TOGGLE(),
                4 => riot_sys::macro_LED4_TOGGLE(),
                5 => riot_sys::macro_LED5_TOGGLE(),
                6 => riot_sys::macro_LED6_TOGGLE(),
                7 => riot_sys::macro_LED7_TOGGLE(),
                _ => unreachable!(),
            }
        };
        Ok(())
    }
}
