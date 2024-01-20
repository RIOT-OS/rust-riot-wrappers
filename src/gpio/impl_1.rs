use super::*;

use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

impl ErrorType for InputGPIO {
    type Error = Infallible;
}

impl InputPin for InputGPIO {
    fn is_high(&mut self) -> Result<bool, Infallible> {
        Ok(unsafe { gpio_read(self.to_c()) } != 0)
    }

    fn is_low(&mut self) -> Result<bool, Infallible> {
        Ok(unsafe { gpio_read(self.to_c()) } == 0)
    }
}

impl ErrorType for OutputGPIO {
    type Error = Infallible;
}

impl OutputPin for OutputGPIO {
    fn set_high(&mut self) -> Result<(), Infallible> {
        unsafe { gpio_set(self.to_c()) };
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Infallible> {
        unsafe { gpio_clear(self.to_c()) };
        Ok(())
    }
}

impl ErrorType for InOutGPIO {
    type Error = Infallible;
}

impl InputPin for InOutGPIO {
    fn is_high(&mut self) -> Result<bool, Infallible> {
        Ok(unsafe { gpio_read(self.to_c()) } != 0)
    }

    fn is_low(&mut self) -> Result<bool, Infallible> {
        Ok(unsafe { gpio_read(self.to_c()) } == 0)
    }
}

impl OutputPin for InOutGPIO {
    fn set_high(&mut self) -> Result<(), Infallible> {
        unsafe { gpio_set(self.to_c()) };
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Infallible> {
        unsafe { gpio_clear(self.to_c()) };
        Ok(())
    }
}
