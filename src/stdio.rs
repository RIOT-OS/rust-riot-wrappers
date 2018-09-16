#[cfg(riot_module_stdio_uart)]
mod regular {
    //! The default implementation of a Rust Stdio object: Write directly to uart_stdio. It does
    //! not go through the C standard library but directly to however uart_stdio is currently
    //! implemented in Riot.

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

            let result = unsafe { stdio_write(transmute(data.as_ptr()), len) };

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

#[cfg(not(riot_module_stdio_uart))]
mod nativestdio {
    //! A fallback implementation of Stdio that goes through the C standard library. That's rather
    //! inefficient as it'd expect null-terminated strings and thus needs to be fed individual
    //! characters, but then again the only platform on which it should be the case that output is
    //! printed even though the STDIO_UART module is not built is native.
    use riot_sys::libc;

    // Is it OK that everyone can instanciate this at any time just so? Probably yes, because the
    // uart_stdio documentation says nothing about limitations on when to call this.
    pub struct Stdio {}

    impl ::core::fmt::Write for Stdio {
        fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
            extern "C" {
                fn putchar(c: libc::c_int) -> libc::c_int;
            }
            s.as_bytes().iter().for_each(|c| unsafe {
                putchar(*c as libc::c_int);
            });
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

#[cfg(not(riot_module_stdio_uart))]
pub use self::nativestdio::Stdio;
#[cfg(riot_module_stdio_uart)]
pub use self::regular::Stdio;
