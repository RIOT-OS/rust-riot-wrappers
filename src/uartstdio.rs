extern crate core;

use core::convert::TryInto;
use core::intrinsics::transmute;

use raw::{
    uart_stdio_write,
};

// Is it OK that everyone can instanciate this at any time just so? Probably yes, because the
// uart_stdio documentation says nothing about limitations on when to call this.
pub struct UartStdio {}

impl core::fmt::Write for UartStdio {
    #[cfg(not(target_os = "linux"))]
    fn write_str(&mut self, s: &str) -> core::fmt::Result
    {
        let data = s.as_bytes();
        // Error here means "Single string too long to be printed in single run". As this can only
        // happen with strings longer than 2**31 bytes, an implementation that writes the string
        // slice-wise is not expected to be ever needed.
        let len: i32 = data.len().try_into().map_err(|_| core::fmt::Error)?;
        let result = unsafe { uart_stdio_write(transmute(data.as_ptr()), len) };

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
