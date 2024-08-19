use super::*;

use embedded_hal_0_2::digital::v2::{InputPin, OutputPin, ToggleableOutputPin};

impl InputPin for InputGPIO {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Infallible> {
        Ok(unsafe { gpio_read(self.to_c()) } != 0)
    }

    fn is_low(&self) -> Result<bool, Infallible> {
        Ok(unsafe { gpio_read(self.to_c()) } == 0)
    }
}

impl OutputPin for OutputGPIO {
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Infallible> {
        unsafe { gpio_set(self.to_c()) };
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Infallible> {
        unsafe { gpio_clear(self.to_c()) };
        Ok(())
    }
}

impl ToggleableOutputPin for OutputGPIO {
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Infallible> {
        unsafe { gpio_toggle(self.to_c()) };
        Ok(())
    }
}

impl InputPin for InOutGPIO {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Infallible> {
        Ok(unsafe { gpio_read(self.to_c()) } != 0)
    }

    fn is_low(&self) -> Result<bool, Infallible> {
        Ok(unsafe { gpio_read(self.to_c()) } == 0)
    }
}

impl OutputPin for InOutGPIO {
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Infallible> {
        unsafe { gpio_set(self.to_c()) };
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Infallible> {
        unsafe { gpio_clear(self.to_c()) };
        Ok(())
    }
}

impl ToggleableOutputPin for InOutGPIO {
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Infallible> {
        unsafe { gpio_toggle(self.to_c()) };
        Ok(())
    }
}
