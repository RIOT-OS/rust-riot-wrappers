use embedded_hal::blocking;
use libc;
use raw::{i2c_t, i2c_acquire, i2c_release, i2c_read_bytes, i2c_write_bytes};

pub struct I2CDevice {
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
}

impl blocking::i2c::WriteRead for I2CDevice
{
    type Error = Error;

    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error>
    {
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
