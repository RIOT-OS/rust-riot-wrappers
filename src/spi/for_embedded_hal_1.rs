//! A rewrite of the SPI wrappers that uses the [`embedded-hal` 1.0 model](embedded_hal::spi) of
//! SPI.
//!
//! This is a dedicated module because the abstractions of that version differ significantly from
//! what was in 0.2 that inspired the original implementation. With the next breaking release of
//! `riot-wrappers`, this module will replace the [`super`] module (possibly with a compatibility
//! alias).
//!
//! # HAL author notes
//!
//! The `embedded-hal` documentations has some [notes for HAL
//! authors](https://docs.rs/embedded-hal/latest/embedded_hal/spi/index.html#for-hal-authors); not
//! all those are followed here:
//!
//! * "HALs **must** implement SpiBus: This is an open FIXME item, which is conditional on a better
//!   understanding of what a RIOT SPI device really means.
//! * "HALS **must not** add infrastructure for sharing at the `SpiBus` level": RIOT already has
//!   that infrastructure; users don't have a guarantee on exclusive, but that's not even SPI
//!   specific: For all it's worth, the SPI hardware can be multiplexed into (say) I2C hardware,
//!   or anything else (hey maybe it's bit banging and the bitbanging core is just busy). At any
//!   rate, this crate is not going out of its way to do that, it's a consequence of being in the
//!   operating system, and technically it's not worse than being preempted by some interrupt.

use crate::error::{NegativeErrorExt, NumericError};
use core::convert::Infallible;
use embedded_hal::spi::{ErrorType, Mode, Operation, SpiDevice};

/// A RIOT SPI device combined with its CS pin, complete with mode and clock configuration.
pub struct SPIDevice {
    bus: riot_sys::spi_t,
    cs: riot_sys::spi_cs_t,
    mode: riot_sys::spi_mode_t,
    clk: riot_sys::spi_clk_t,
}

impl SPIDevice {
    /// Creates a new SPI device, given its RIOT bus number (equivalent to running
    /// `SPI_DEV(number)`) and its CS GPIO pin.
    ///
    /// By default, the clock speed is set to the lowest speed supported by the hardware, and the
    /// mode is set to SPI mode number 0 (the most common one).
    #[cfg(riot_module_periph_gpio)]
    pub fn from_number_and_cs_pin(
        number: u32,
        cs: crate::gpio::GPIO,
    ) -> Result<Self, NumericError> {
        // SAFETY: This is designed to be called with any number. (Whether the device is then valid
        // will show later).
        let bus = unsafe { riot_sys::macro_SPI_DEV(number) };
        let cs = cs.to_c();
        (unsafe { riot_sys::spi_init_cs(bus, cs) }).negative_to_error()?;
        Ok(Self {
            bus,
            cs,
            mode: riot_sys::spi_mode_t_SPI_MODE_0,
            clk: riot_sys::spi_clk_t_SPI_CLK_100KHZ,
        })
    }

    // This family of speed setters is deliberately by-function, because this can easily be kept
    // available no matter how RIOT decides to support arbitrary speeds.

    /// Sets the speed to 100KHz.
    pub fn with_speed_100khz(self) -> Self {
        Self {
            clk: riot_sys::spi_clk_t_SPI_CLK_100KHZ,
            ..self
        }
    }

    /// Sets the speed to 400KHz.
    pub fn with_speed_400khz(self) -> Self {
        Self {
            clk: riot_sys::spi_clk_t_SPI_CLK_100KHZ,
            ..self
        }
    }

    /// Sets the speed to 1MHz.
    pub fn with_speed_1mhz(self) -> Self {
        Self {
            clk: riot_sys::spi_clk_t_SPI_CLK_1MHZ,
            ..self
        }
    }

    /// Sets the speed to 5MHz.
    pub fn with_speed_5mhz(self) -> Self {
        Self {
            clk: riot_sys::spi_clk_t_SPI_CLK_5MHZ,
            ..self
        }
    }

    /// Sets the speed to 10MHz.
    pub fn with_speed_10mhz(self) -> Self {
        Self {
            clk: riot_sys::spi_clk_t_SPI_CLK_10MHZ,
            ..self
        }
    }

    /// Sets the device's mode.
    pub fn with_mode(self, mode: Mode) -> Self {
        Self {
            mode: match mode {
                embedded_hal::spi::MODE_0 => riot_sys::spi_mode_t_SPI_MODE_0,
                embedded_hal::spi::MODE_1 => riot_sys::spi_mode_t_SPI_MODE_1,
                embedded_hal::spi::MODE_2 => riot_sys::spi_mode_t_SPI_MODE_2,
                embedded_hal::spi::MODE_3 => riot_sys::spi_mode_t_SPI_MODE_3,
            },
            ..self
        }
    }
}

impl ErrorType for SPIDevice {
    type Error = core::convert::Infallible;
}

impl SpiDevice for SPIDevice {
    // No need to implement flush(): It's not very explicit, but as spi_release() docs say that
    // after release, the SPI bus should be powered down, that only works if release blocks until
    // that is done.

    fn transaction(&mut self, ops: &mut [Operation<'_, u8>]) -> Result<(), Infallible> {
        unsafe { riot_sys::spi_acquire(self.bus, self.cs, self.mode, self.clk) };
        let len = ops.len();
        for (index, op) in ops.iter_mut().enumerate() {
            let cont = index != len - 1;
            match op {
                Operation::Read(bytes) => unsafe {
                    riot_sys::spi_transfer_bytes(
                        self.bus,
                        self.cs,
                        cont,
                        core::ptr::null(),
                        bytes.as_mut_ptr() as _,
                        bytes.len().try_into().expect("usize and size_t match"),
                    );
                },
                Operation::Write(bytes) => unsafe {
                    riot_sys::spi_transfer_bytes(
                        self.bus,
                        self.cs,
                        cont,
                        bytes.as_ptr() as _,
                        core::ptr::null_mut(),
                        bytes.len().try_into().expect("usize and size_t match"),
                    );
                },
                Operation::Transfer(read, write) => unsafe {
                    use core::cmp::{max, min};
                    // Or would this be expressed more easily as the 3 cases "same length", "one
                    // longer" and "the other longer"?
                    let first_part = min(read.len(), write.len());
                    let second_part = max(read.len(), write.len()) - first_part;
                    riot_sys::spi_transfer_bytes(
                        self.bus,
                        self.cs,
                        cont || (second_part > 0),
                        write.as_ptr() as _,
                        read.as_mut_ptr() as _,
                        first_part.try_into().expect("usize and size_t match"),
                    );
                    if second_part > 0 {
                        riot_sys::spi_transfer_bytes(
                            self.bus,
                            self.cs,
                            cont,
                            if write.len() == first_part {
                                core::ptr::null()
                            } else {
                                write[first_part..].as_ptr() as _
                            },
                            if read.len() == first_part {
                                core::ptr::null_mut()
                            } else {
                                read[first_part..].as_mut_ptr() as _
                            },
                            second_part.try_into().expect("usize and size_t match"),
                        );
                    }
                },
                Operation::TransferInPlace(bytes) => unsafe {
                    riot_sys::spi_transfer_bytes(
                        self.bus,
                        self.cs,
                        cont,
                        bytes.as_ptr() as _,
                        bytes.as_mut_ptr() as _,
                        bytes.len().try_into().expect("usize and size_t match"),
                    );
                },
                Operation::DelayNs(time) => {
                    crate::ztimer::Clock::usec().sleep(crate::ztimer::Ticks(time.div_ceil(1000)));
                }
            }
        }
        // SAFETY: as per C API.
        unsafe { riot_sys::spi_release(self.bus) };
        Ok(())
    }
}
