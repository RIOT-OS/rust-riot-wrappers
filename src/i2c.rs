//! Controlling the I²C bus

use embedded_hal::blocking;
use riot_sys::i2c_t;

/// An I²C master backed by RIOT's [I2C implementation]
///
/// [I2C implementation]: http://doc.riot-os.org/group__drivers__periph__i2c.html
///
/// Actual transactions on this are performed through the [embedded_hal::blocking::i2c] traits
/// implemented by this.
#[derive(Debug)]
pub struct I2CDevice {
    dev: i2c_t,
}

impl I2CDevice {
    /// Create a new I2CDevice from a RIOT descriptor
    ///
    /// As all transactions on the bus are gated by acquire / release steps implied in the
    /// individual reads or writes, multiple copies of the same device can safely coexist.
    pub fn new(dev: i2c_t) -> Self {
        I2CDevice { dev }
    }
}

#[derive(Debug)]
pub enum Error {
    AcquireError,
    WriteError(i32),
    ReadError(i32),
    #[deprecated]
    /// State returned in earlier versions that built the I2C module even if absent from RIOT
    DeviceNotFound,
}

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
                bytes.len() as _,
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
                buffer.len() as _,
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
                bytes.len() as _,
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
                buffer.len() as _,
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
