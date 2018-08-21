// Manually adapted from the output of
//
//     bindgen ../RIOT/drivers/include/periph/i2c.h --use-core --ctypes-prefix=libc -o i2c.rs -- -I ../RIOT/sys/include -I ../RIOT/drivers/include -I ../RIOT/core/include -I .

use libc;

pub type i2c_t = libc::c_uint;

extern "C" {
    pub fn i2c_init(dev: i2c_t);
    pub fn i2c_acquire(dev: i2c_t) -> libc::c_int;
    pub fn i2c_release(dev: i2c_t) -> libc::c_int;
    pub fn i2c_read_bytes(
        dev: i2c_t,
        addr: u16,
        data: *mut libc::c_void,
        len: usize,
        flags: u8,
    ) -> libc::c_int;
    pub fn i2c_write_bytes(
        dev: i2c_t,
        addr: u16,
        data: *const libc::c_void,
        len: usize,
        flags: u8,
    ) -> libc::c_int;
}

// Again, this is the part that'll be split out

use embedded_hal::blocking;

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
