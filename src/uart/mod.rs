//! Wrapper for using the UART
use crate::error::{NegativeErrorExt, NumericError};
use embedded_io;
use embedded_io::ErrorKind;
use riot_sys::libc::c_void;
use riot_sys::macro_UART_DEV;
use riot_sys::uart_init;
use riot_sys::uart_t;
use riot_sys::uart_write;

pub struct UARTDevice {
    dev: uart_t,
}

impl embedded_io::Error for NumericError {
    fn kind(&self) -> ErrorKind {
        match -self.number() as _ {
            riot_sys::ENODEV => ErrorKind::NotFound,
            riot_sys::ENOTSUP => ErrorKind::Unsupported,
            _ => ErrorKind::Other,
        }
    }
}

impl embedded_io::ErrorType for UARTDevice {
    type Error = NumericError;
}

impl UARTDevice {
    /// Create a new UARTDevice from a RIOT descriptor
    pub fn from_c(dev: uart_t) -> Self {
        UARTDevice { dev }
    }

    /// Create a new UARTDevice from device number
    pub fn from_port(dev: u32) -> Self {
        UARTDevice {
            dev: unsafe { macro_UART_DEV(dev) },
        }
    }

    unsafe extern "C" fn cb<F: FnMut(u8) + Send + 'static>(ctx: *mut c_void, byte: u8) {
        let real_cb: &'static mut F = unsafe { &mut *(ctx as *mut _) };
        (real_cb)(byte);
    }

    pub fn init_with_fn<F: FnMut(u8) + Send + 'static>(
        &mut self,
        baudrate: u32,
        callback: &'static mut F,
    ) -> Result<(), NumericError> {
        let result = {
            unsafe {
                uart_init(
                    self.dev,
                    baudrate,
                    Some(UARTDevice::cb::<F>),
                    callback as *const _ as *mut _,
                )
            }
        };

        result
            .negative_to_error()
            .map_or_else(|err| Err(NumericError::from(err)), |_| Ok(()))
    }

    pub fn init_with_closure<F: FnMut(u8) + Send + 'static>(
        &mut self,
        baudrate: u32,
        callback: F,
    ) -> Result<(), NumericError> {
        let result = {
            unsafe {
                uart_init(
                    self.dev,
                    baudrate,
                    Some(UARTDevice::cb::<F>),
                    &callback as *const _ as *mut _,
                )
            }
        };

        result
            .negative_to_error()
            .map_or_else(|err| Err(NumericError::from(err)), |_| Ok(()))
    }
}

impl embedded_io::Write for UARTDevice {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        unsafe { uart_write(self.dev, buf.as_ptr(), buf.len() as u32) };
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
