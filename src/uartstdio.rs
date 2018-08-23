extern crate core;

use core::convert::TryInto;

extern "C" {
    fn uart_stdio_read(buffer: *mut [u8], len: isize) -> isize;
    fn uart_stdio_write(buffer: *const [u8], len: isize) -> isize;
}

// Is it OK that everyone can instanciate this at any time just so? Probably yes, because the
// uart_stdio documentation says nothing about limitations on when to call this.
pub struct UartStdio {}

impl core::fmt::Write for UartStdio {
    #[cfg(not(target_os = "linux"))]
    fn write_str(&mut self, s: &str) -> core::fmt::Result
    {
        let data = s.as_bytes();
        let len: isize = data.len().try_into().unwrap();
        let result = unsafe { uart_stdio_write(data, len) };

        if result == len {
            Ok(())
        } else {
            Err(core::fmt::Error)
        }
    }

    // FIXME have a better criterion
    #[cfg(target_os = "linux")]
    fn write_str(&mut self, s: &str) -> core::fmt::Result
    {
        extern "C" {
            fn putchar(c: isize) -> isize;
        }
        s.as_bytes().iter().for_each(|c| unsafe {putchar(*c as isize);});
        Ok(())
    }
}
