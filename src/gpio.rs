use riot_sys::{gpio_clear, gpio_read, gpio_set, gpio_t, gpio_toggle};

use embedded_hal::digital::v2::{InputPin, OutputPin, ToggleableOutputPin};

/// A Rust representation of RIOT's gpio_t, representing a single pin in no particular
/// configuration.
pub struct GPIO(gpio_t);

impl GPIO {
    /// Create a GPIO from a gpio_t
    ///
    /// This is as safe as any device acquisition from C is -- RIOT's drivers are (hopefully)
    /// written in such a way that concurrent writes to adjacent pins don't interfere, and those to
    /// the same device are "just" racy.
    pub fn from_c(gpio: gpio_t) -> Option<Self> {
        if unsafe { riot_sys::gpio_is_valid(gpio) } != 0 {
            Some(GPIO(gpio))
        } else {
            None
        }
    }

    // using a generic GPIO_PIN is probably best done by making GPIO_INIT a static inline (given
    // it's already fixed to types at tests/periph_gpio/main.c)
    //     /// Create a GPIO out of thin air
    //     #[cfg(riot_module_nrf5x_common_periph)]
    //     pub unsafe fn new(port: u8, pin: u8) -> Self {
    //         // EXPANDED cpu/nrf5x_common/include/periph_cpu_common.h:50
    //         GPIO(((port << 5) | pin).into())
    //     }

    pub unsafe fn as_output(self) -> OutputGPIO {
        // FIXME should we configure here? it's probably even safe
        OutputGPIO(self)
    }

    pub unsafe fn as_input(self) -> InputGPIO {
        // FIXME should we configure here? it's probably even safe
        InputGPIO(self)
    }

    /// Get a gpio_t from a configured pin
    ///
    /// This is typically useful when populating a RIOT mechanism that works on a pre-configured
    /// pin.
    pub fn to_c(&self) -> riot_sys::gpio_t {
        self.0
    }
}

pub struct OutputGPIO(GPIO);

impl OutputGPIO {
    /// See [GPIO::to_c]
    pub fn to_c(&self) -> riot_sys::gpio_t {
        self.0.to_c()
    }
}

impl OutputPin for OutputGPIO {
    type Error = !;

    fn set_high(&mut self) -> Result<(), !> {
        unsafe { gpio_set(self.to_c()) };
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), !> {
        unsafe { gpio_clear(self.to_c()) };
        Ok(())
    }
}

impl ToggleableOutputPin for OutputGPIO {
    type Error = !;

    fn toggle(&mut self) -> Result<(), !> {
        unsafe { gpio_toggle(self.to_c()) };
        Ok(())
    }
}

pub struct InputGPIO(GPIO);

impl InputGPIO {
    /// See [GPIO::to_c]
    pub fn to_c(&self) -> riot_sys::gpio_t {
        self.0.to_c()
    }
}

impl InputPin for InputGPIO {
    type Error = !;

    fn is_high(&self) -> Result<bool, !> {
        Ok(unsafe { gpio_read(self.to_c()) } != 0)
    }

    fn is_low(&self) -> Result<bool, !> {
        Ok(unsafe { gpio_read(self.to_c()) } == 0)
    }
}
