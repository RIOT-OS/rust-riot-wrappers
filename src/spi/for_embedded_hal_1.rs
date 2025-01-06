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
//! * "HALS **must not** add infrastructure for sharing at the `SpiBus` level": RIOT already has
//!   that infrastructure; users don't have a guarantee on exclusive, but that's not even SPI
//!   specific: For all it's worth, the SPI hardware can be multiplexed into (say) I2C hardware,
//!   or anything else (hey maybe it's bit banging and the bitbanging core is just busy). At any
//!   rate, this crate is not going out of its way to do that, it's a consequence of being in the
//!   operating system, and technically it's not worse than being preempted by some interrupt.

use crate::error::{NegativeErrorExt, NumericError};
use core::convert::Infallible;
use embedded_hal::spi::{ErrorType, Mode, Operation, SpiBus, SpiDevice};

/// A RIOT SPI device combined with complete with mode and clock configuration, but no particular
/// CS pin.
///
/// Note that while this implements [`embedded-hal::SpiBus`], it is not exclusive (because no
/// peripheral in RIOT is); when accessed while another "owner" uses it, operations only start when
/// the other party is done.
pub struct SPIBus {
    bus: riot_sys::spi_t,
    mode: riot_sys::spi_mode_t,
    clk: riot_sys::spi_clk_t,
}

/// A RIOT SPI device combined with its CS pin, complete with mode and clock configuration.
pub struct SPIDevice {
    bus: SPIBus,
    cs: riot_sys::spi_cs_t,
}

impl SPIBus {
    /// Creates a new SPI device, given its RIOT bus number (equivalent to running
    /// `SPI_DEV(number)`).
    ///
    /// By default, the clock speed is set to the lowest speed supported by the hardware, and the
    /// mode is set to SPI mode number 0 (the most common one).
    pub fn from_number(number: u32) -> Self {
        // SAFETY: This is designed to be called with any number. (Whether the device is then valid
        // will show later).
        let bus = unsafe { riot_sys::macro_SPI_DEV(number) };
        Self {
            bus,
            mode: riot_sys::spi_mode_t_SPI_MODE_0,
            clk: riot_sys::spi_clk_t_SPI_CLK_100KHZ,
        }
    }

    /// Convenience alias for [`SPIDevice::new()`] for builder style construction.
    #[cfg(riot_module_periph_gpio)]
    pub fn with_cs(self, cs: crate::gpio::GPIO) -> Result<SPIDevice, NumericError> {
        SPIDevice::new(self, cs)
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
            clk: riot_sys::spi_clk_t_SPI_CLK_400KHZ,
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

impl ErrorType for SPIBus {
    type Error = core::convert::Infallible;
}

// To avoid re-implementing everything and especially the split transfer logic, this is hooking
// into the transaction function, which is largely modeled after SpiDevice's API.
//
// This may or may not be efficient depending on what the compiler inlines, but really, if you want
// efficient, go with SpiDevice anyway for hardware CS. Another downside of this approach is that
// it means that ZTimer is a dependency even when not using SpiDevice; sticking with it for overall
// simplicity.
impl SpiBus for SPIBus {
    fn read(
        &mut self,
        words: &mut [u8],
    ) -> Result<(), <Self as embedded_hal::spi::ErrorType>::Error> {
        transaction(
            &self,
            riot_sys::inline::GPIO_UNDEF.try_into().unwrap(),
            &mut [Operation::Read(words)],
        );
        Ok(())
    }
    fn write(&mut self, words: &[u8]) -> Result<(), <Self as embedded_hal::spi::ErrorType>::Error> {
        transaction(
            &self,
            riot_sys::inline::GPIO_UNDEF.try_into().unwrap(),
            &mut [Operation::Write(words)],
        );
        Ok(())
    }
    fn transfer(
        &mut self,
        read: &mut [u8],
        write: &[u8],
    ) -> Result<(), <Self as embedded_hal::spi::ErrorType>::Error> {
        transaction(
            &self,
            riot_sys::inline::GPIO_UNDEF.try_into().unwrap(),
            &mut [Operation::Transfer(read, write)],
        );
        Ok(())
    }
    fn transfer_in_place(
        &mut self,
        words: &mut [u8],
    ) -> Result<(), <Self as embedded_hal::spi::ErrorType>::Error> {
        transaction(
            &self,
            riot_sys::inline::GPIO_UNDEF.try_into().unwrap(),
            &mut [Operation::TransferInPlace(words)],
        );
        Ok(())
    }
    fn flush(&mut self) -> Result<(), <Self as embedded_hal::spi::ErrorType>::Error> {
        // See also comment on `SpiDevice for SPIDevice`
        Ok(())
    }
}

impl SPIDevice {
    /// and its CS GPIO pin
    #[cfg(riot_module_periph_gpio)]
    pub fn new(bus: SPIBus, cs: crate::gpio::GPIO) -> Result<Self, NumericError> {
        let cs = cs.to_c();
        (unsafe { riot_sys::spi_init_cs(bus.bus, cs) }).negative_to_error()?;
        Ok(Self { bus, cs })
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
        transaction(&self.bus, self.cs, ops);
        Ok(())
    }
}

fn transaction(bus: &SPIBus, cs: riot_sys::spi_cs_t, ops: &mut [Operation<'_, u8>]) {
    unsafe { riot_sys::spi_acquire(bus.bus, cs, bus.mode, bus.clk) };
    let len = ops.len();
    for (index, op) in ops.iter_mut().enumerate() {
        let cont = index != len - 1;
        match op {
            Operation::Read(bytes) => unsafe {
                riot_sys::spi_transfer_bytes(
                    bus.bus,
                    cs,
                    cont,
                    core::ptr::null(),
                    bytes.as_mut_ptr() as _,
                    bytes.len().try_into().expect("usize and size_t match"),
                );
            },
            Operation::Write(bytes) => unsafe {
                riot_sys::spi_transfer_bytes(
                    bus.bus,
                    cs,
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
                let read_longer = read.len() > write.len();
                let write_longer = write.len() > read.len();
                riot_sys::spi_transfer_bytes(
                    bus.bus,
                    cs,
                    cont || (second_part > 0),
                    write.as_ptr() as _,
                    read.as_mut_ptr() as _,
                    first_part.try_into().expect("usize and size_t match"),
                );
                if read_longer {
                 riot_sys::spi_transfer_bytes(
                        bus.bus,
                        cs,
                        cont,
                        core::ptr::null(),
                        read[write.len()..].as_mut_ptr() as _,
                        (read.len()-write.len()).try_into().expect("usize and size_t match"),
                    );
                }
                if write_longer {
                    riot_sys::spi_transfer_bytes(
                        bus.bus,
                        cs,
                        cont,
                        write[read.len()..].as_ptr() as _,
                        core::ptr::null_mut(),
                         (write.len()-read.len()).try_into().expect("usize and size_t match"),
                    );
                }
            },
            Operation::TransferInPlace(bytes) => unsafe {
                riot_sys::spi_transfer_bytes(
                    bus.bus,
                    cs,
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
    unsafe { riot_sys::spi_release(bus.bus) };
}
