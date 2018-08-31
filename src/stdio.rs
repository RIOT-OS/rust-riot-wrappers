#[cfg(not(target_os = "linux"))]
mod regular {
    use ::core::intrinsics::transmute;
    use raw::{
        stdio_write,
        stdio_read,
    };

    // Is it OK that everyone can instanciate this at any time just so? Probably yes, because the
    // uart_stdio documentation says nothing about limitations on when to call this.
    pub struct Stdio {}


    impl ::core::fmt::Write for Stdio {
        fn write_str(&mut self, s: &str) -> ::core::fmt::Result
        {
            let data = s.as_bytes();
            let result = unsafe { stdio_write(transmute(data.as_ptr()), data.len()) };

            if result >= 0 {
                Ok(())
            } else {
                Err(::core::fmt::Error)
            }
        }
    }

    impl Stdio {
        pub fn read_raw<'a>(&mut self, buffer: &'a mut [u8]) -> Result<&'a mut [u8], ()> {
            let bytes_read = unsafe { stdio_read(transmute(buffer.as_mut_ptr()), buffer.len()) };
            if bytes_read >= 0 {
                Ok(&mut buffer[..bytes_read as usize])
            } else {
                Err(())
            }
        }
    }
}

// FIXME have a better criterion
#[cfg(target_os = "linux")]
mod nativestdio {
    use libc;

    // Is it OK that everyone can instanciate this at any time just so? Probably yes, because the
    // uart_stdio documentation says nothing about limitations on when to call this.
    pub struct Stdio {}

    impl ::core::fmt::Write for Stdio {
        fn write_str(&mut self, s: &str) -> ::core::fmt::Result
        {
            extern "C" {
                fn putchar(c: libc::c_int) -> libc::c_int;
            }
            s.as_bytes().iter().for_each(|c| unsafe {putchar(*c as libc::c_int);});
            Ok(())
        }
    }

    impl Stdio {
        pub fn read_raw<'a>(&mut self, buffer: &'a mut [u8]) -> Result<&'a mut [u8], ()> {
            extern "C" {
                fn getchar() -> libc::c_int;
            }
            // This always reads exactly one character, unlike the UART that often reads many, but
            // that's not wrong either.
            let actually_read = unsafe { getchar() };

            let first = &mut buffer[..1];
            first[0] = actually_read as u8;
            Ok(first)
        }
    }
}

#[cfg(not(target_os = "linux"))]
pub use self::regular::Stdio;
#[cfg(target_os = "linux")]
pub use self::nativestdio::Stdio;
