extern crate core;

use core::intrinsics::transmute;

use raw::{
    stdio_write,
};

// Is it OK that everyone can instanciate this at any time just so? Probably yes, because the
// uart_stdio documentation says nothing about limitations on when to call this.
pub struct Stdio {}

impl core::fmt::Write for Stdio {
     #[cfg(not(target_os = "linux"))]
    fn write_str(&mut self, s: &str) -> core::fmt::Result
    {
        let data = s.as_bytes();
        let result = unsafe { stdio_write(transmute(data.as_ptr()), data.len()) };

        if result >= 0 {
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
