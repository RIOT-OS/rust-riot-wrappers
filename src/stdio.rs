use core::intrinsics::transmute;
use riot_sys::{stdio_read, stdio_write};

// Is it OK that everyone can instanciate this at any time just so? Probably yes, because the
// uart_stdio documentation says nothing about limitations on when to call this.
pub struct Stdio {}

impl ::core::fmt::Write for Stdio {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        let data = s.as_bytes();
        let len = data.len();
        if len == 0 {
            return Ok(());
        }

        let result = unsafe { stdio_write(transmute(data.as_ptr()), len as _) };

        if result >= 0 {
            Ok(())
        } else {
            Err(::core::fmt::Error)
        }
    }
}

impl Stdio {
    pub fn read_raw<'a>(&mut self, buffer: &'a mut [u8]) -> Result<&'a mut [u8], ()> {
        let bytes_read = unsafe { stdio_read(transmute(buffer.as_mut_ptr()), buffer.len() as _) };
        if bytes_read >= 0 {
            Ok(&mut buffer[..bytes_read as usize])
        } else {
            Err(())
        }
    }
}

// Copied and adapted from Rust 1.32.0
#[macro_export]
macro_rules! dbg {
    ($val:expr) => {
        match $val {
            tmp => {
                use core::fmt::Write;
                use $crate::stdio::Stdio;
                let _ = writeln!(
                    Stdio {},
                    "[{}:{}] {} = {:#?}",
                    file!(),
                    line!(),
                    stringify!($val),
                    &tmp
                );
                tmp
            }
        }
    };
}

pub use dbg;
