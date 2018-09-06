use embedded_hal::blocking;
use riot_sys::i2c_t;

pub struct I2CDevice {
    // because the not_actually_i2c implementation does not use it, but I still want to keep the
    // signatures the same and re-use initialization.
    #[allow(dead_code)]
    dev: i2c_t,
}

impl I2CDevice
{
    pub fn new(dev: i2c_t) -> Self
    {
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

#[cfg(not(target_os="linux"))]
mod regular {
    use super::*;
    use riot_sys::{i2c_acquire, i2c_release, i2c_read_bytes, i2c_write_bytes, I2C_COUNT};
    use riot_sys::libc;

    impl blocking::i2c::WriteRead for I2CDevice
    {
        type Error = Error;

        fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error>
        {
            // this doesn't really work yet as it doesn't remove the links to i2c_write_bytes (which
            // are not present in native as no i2c device is here).
            if I2C_COUNT == 0 {
                return Err(Error::DeviceNotFound);
            }

            let err = unsafe { i2c_acquire(self.dev) };
            if err != 0 {
                return Err(Error::AcquireError);
            }
            let err = unsafe { i2c_write_bytes(self.dev, address as u16, bytes.as_ptr() as *const libc::c_void, bytes.len(), 0) };
            if err != 0 {
                unsafe { i2c_release(self.dev) };
                return Err(Error::WriteError(err));
            }
            let err = unsafe { i2c_read_bytes(self.dev, address as u16, buffer.as_ptr() as *mut libc::c_void, buffer.len(), 0) };
            if err != 0 {
                unsafe { i2c_release(self.dev) };
                return Err(Error::ReadError(err));
            }
            unsafe { i2c_release(self.dev) };
            Ok(())
        }
    }

}

#[cfg(target_os="linux")]
mod not_actually_i2c {
    use super::*;

    impl blocking::i2c::WriteRead for I2CDevice
    {
        type Error = Error;

        // for the native board which has no I2C; the discriminator is a terrible choice but works
        // right now; ideally, the above solution with I2C_COUNT gating would work, but if that's a
        // dead end, the next best options are pulling some #define-s out into cfg options.
        #[cfg(target_os="linux")]
        fn write_read(&mut self, _address: u8, _bytes: &[u8], _buffer: &mut [u8]) -> Result<(), Self::Error>
        {
            Err(Error::DeviceNotFound)
        }
    }
}
