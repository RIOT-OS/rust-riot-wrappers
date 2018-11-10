use embedded_hal::blocking;
use riot_sys::i2c_t;

pub struct I2CDevice {
    // because the not_actually_i2c implementation does not use it, but I still want to keep the
    // signatures the same and re-use initialization.
    #[allow(dead_code)]
    dev: i2c_t,
}

impl I2CDevice {
    pub fn new(dev: i2c_t) -> Self {
        I2CDevice { dev }
    }
}

#[derive(Debug)]
pub enum Error {
    AcquireError,
    WriteError(i32),
    ReadError(i32),
    // that's messy; to be cleaned up when the returned errors are actually interpreted
    DeviceNotFound,
}

#[cfg(riot_module_periph_i2c)]
mod regular {
    use super::*;
    use riot_sys::libc;
    use riot_sys::{i2c_acquire, i2c_read_bytes, i2c_release, i2c_write_bytes};

    impl blocking::i2c::WriteRead for I2CDevice {
        type Error = Error;

        fn write_read(
            &mut self,
            address: u8,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            let err = unsafe { i2c_acquire(self.dev) };
            if err != 0 {
                return Err(Error::AcquireError);
            }
            let err = unsafe {
                i2c_write_bytes(
                    self.dev,
                    address as u16,
                    bytes.as_ptr() as *const libc::c_void,
                    bytes.len(),
                    0,
                )
            };
            if err != 0 {
                unsafe { i2c_release(self.dev) };
                return Err(Error::WriteError(err));
            }
            let err = unsafe {
                i2c_read_bytes(
                    self.dev,
                    address as u16,
                    buffer.as_ptr() as *mut libc::c_void,
                    buffer.len(),
                    0,
                )
            };
            if err != 0 {
                unsafe { i2c_release(self.dev) };
                return Err(Error::ReadError(err));
            }
            unsafe { i2c_release(self.dev) };
            Ok(())
        }
    }

    impl blocking::i2c::Write for I2CDevice {
        type Error = Error;

        fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            let err = unsafe { i2c_acquire(self.dev) };
            if err != 0 {
                return Err(Error::AcquireError);
            }
            let err = unsafe {
                i2c_write_bytes(
                    self.dev,
                    address as u16,
                    bytes.as_ptr() as *const libc::c_void,
                    bytes.len(),
                    0,
                )
            };
            if err != 0 {
                unsafe { i2c_release(self.dev) };
                return Err(Error::WriteError(err));
            }
            unsafe { i2c_release(self.dev) };
            Ok(())
        }
    }

    impl blocking::i2c::Read for I2CDevice {
        type Error = Error;

        fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            let err = unsafe { i2c_acquire(self.dev) };
            if err != 0 {
                return Err(Error::AcquireError);
            }
            let err = unsafe {
                i2c_read_bytes(
                    self.dev,
                    address as u16,
                    buffer.as_ptr() as *mut libc::c_void,
                    buffer.len(),
                    0,
                )
            };
            if err != 0 {
                unsafe { i2c_release(self.dev) };
                return Err(Error::ReadError(err));
            }
            unsafe { i2c_release(self.dev) };
            Ok(())
        }
    }
}

#[cfg(not(riot_module_periph_i2c))]
mod not_actually_i2c {
    use super::*;

    impl blocking::i2c::WriteRead for I2CDevice {
        type Error = Error;

        fn write_read(
            &mut self,
            _address: u8,
            _bytes: &[u8],
            _buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            Err(Error::DeviceNotFound)
        }
    }

    impl blocking::i2c::Write for I2CDevice {
        type Error = Error;

        fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            Err(Error::DeviceNotFound)
        }
    }

    impl blocking::i2c::Read for I2CDevice {
        type Error = Error;

        fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            Err(Error::DeviceNotFound)
        }
    }
}
