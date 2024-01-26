//! Implementation of embedded-hal 0.2's I2C for [I2CDevice]
//!
//! As the implementation is on the [I2CDevice directly], all that is in this module is the
//! suitable [Error] type.

use embedded_hal_0_2::blocking;

use super::*;

use riot_sys::libc;
use riot_sys::{i2c_acquire, i2c_read_bytes, i2c_release, i2c_write_bytes};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    AcquireError,
    WriteError(i32),
    ReadError(i32),
}

impl blocking::i2c::WriteRead for I2CDevice {
    type Error = Error;

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        unsafe { i2c_acquire(self.dev) };
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
        unsafe { i2c_acquire(self.dev) };
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
        unsafe { i2c_acquire(self.dev) };
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
