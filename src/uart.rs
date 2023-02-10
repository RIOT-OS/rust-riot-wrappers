//! Access to [RIOT's UART](https://doc.riot-os.org/group__drivers__periph__uart.html)
//!
//! Author: Kilian Barning <barning@uni-bremen.de>
#![allow(dead_code)]

use core::marker::PhantomData;
use core::ptr;

use riot_sys::libc::{c_int, c_void};
use riot_sys::*;

/// This enum representatives the status returned by various `UART`-functions
#[derive(Debug)]
#[non_exhaustive]
pub enum UartDeviceStatus {
    Success,
    InvalidDevice,
    UnsupportedConfig,
    Other(i32),
}

impl UartDeviceStatus {
    /// Converts the given `c_int` into the matching Enum representation
    fn from_c(n: c_int) -> Self {
        const _ENODEV: c_int = -(ENODEV as c_int);
        const _ENOTSUP: c_int = -(ENOTSUP as c_int);
        match n {
            0 => Self::Success,
            _ENODEV => Self::InvalidDevice,
            _ENOTSUP => Self::UnsupportedConfig,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug)]
pub enum DataBits {
    Five,
    Six,
    Seven,
    Eight,
}

impl DataBits {
    fn to_c(self) -> uart_data_bits_t {
        match self {
            Self::Five => uart_data_bits_t_UART_DATA_BITS_5,
            Self::Six => uart_data_bits_t_UART_DATA_BITS_6,
            Self::Seven => uart_data_bits_t_UART_DATA_BITS_7,
            Self::Eight => uart_data_bits_t_UART_DATA_BITS_8,
        }
    }
}

#[derive(Debug)]
pub enum Parity {
    None,
    Even,
    Odd,
    Mark,
    Space,
}

impl Parity {
    fn to_c(self) -> uart_parity_t {
        match self {
            Self::None => uart_parity_t_UART_PARITY_NONE,
            Self::Even => uart_parity_t_UART_PARITY_EVEN,
            Self::Odd => uart_parity_t_UART_PARITY_ODD,
            Self::Mark => uart_parity_t_UART_PARITY_MARK,
            Self::Space => uart_parity_t_UART_PARITY_SPACE,
        }
    }
}


#[derive(Debug)]
pub enum StopBits {
    One,
    Two,
}

impl StopBits {
    fn to_c(self) -> uart_stop_bits_t {
        match self {
            Self::One => uart_stop_bits_t_UART_STOP_BITS_1,
            Self::Two => uart_stop_bits_t_UART_STOP_BITS_2,
        }
    }
}

/// This struct contains the `UART` device and handles all operation regarding it
///
/// [UART implementation]: https://doc.riot-os.org/group__drivers__periph__uart.html
#[derive(Debug)]
pub struct UartDevice<'a> {
    dev: uart_t,
    _marker: PhantomData<&'a ()>, // We use this `PhantomData` here to make sure that the lifetime of the borrowed closure is equal to this struct
}

impl<'a> UartDevice<'a> {
    /// Tries to initialize the given `UART`. Returns a Result with rather `Ok<Self>` if the UART was initialized successfully or a
    /// `Err<UartDeviceStatus>` containing the error
    ///
    /// # Arguments
    ///
    /// * `dev` - The uart_t handle to the hardware device
    /// * `baud` - The used baud rate
    /// * `user_callback` The user defined callback that gets called from the os whenever new data is received from the `UART`
    ///
    /// # Examples
    /// ```
    /// use riot_wrappers::uart::UartDevice;
    /// let mut cb = |new_data| {
    ///     //do something here with the received data
    /// };
    /// let mut uart = UartDevice::new(uart_type_t_STM32_USART, 115200, &mut cb)
    ///     .unwrap_or_else(|e| panic!("Error initializing UART: {e:?}"));
    /// uart.write(b"Hello from UART");
    /// ```
    pub fn new<F>(
        dev: uart_t,
        baud: u32,
        user_callback: &'a mut F,
    ) -> Result<Self, UartDeviceStatus>
    where
        F: FnMut(u8),
    {
        unsafe {
            match UartDeviceStatus::from_c(uart_init(
                dev,
                baud,
                Some(Self::new_data_callback::<F>),
                user_callback as *mut _ as *mut c_void,
            )) {
                UartDeviceStatus::Success => Ok(Self {
                    dev,
                    _marker: PhantomData,
                }),
                status => Err(status),
            }
        }
    }

    /// Tries to initialize the given `UART`. Returns a Result with rather `Ok<Self>` if the UART was initialized successfully or a
    /// `Err<UartDeviceStatus>` containing the error. As the name implies, the created `UART` device can <b>ONLY</b> send data
    ///
    /// # Arguments
    ///
    /// * `dev` - The uart_t handle to the hardware device
    /// * `baud` - The used baud rate
    ///
    /// # Examples
    /// ```
    /// use riot_wrappers::uart::UartDevice;
    /// let mut uart = UartDevice::new_without_rx(uart_type_t_STM32_USART, 115200)
    /// .unwrap_or_else(|e| panic!("Error initializing UART: {e:?}"));
    /// uart.write(b"Hello from UART");
    /// ```
    pub fn new_without_rx(dev: uart_t, baud: u32) -> Result<Self, UartDeviceStatus> {
        unsafe {
            match UartDeviceStatus::from_c(uart_init(dev, baud, None, ptr::null_mut())) {
                UartDeviceStatus::Success => Ok(Self {
                    dev,
                    _marker: PhantomData,
                }),
                status => Err(status),
            }
        }
    }


    /// Sets the mode according to the given parameters
    /// Should the parameters be invalid, the function returns a Err<UartDeviceStatus::UnsupportedConfig>
    /// # Arguments
    /// * `data_bits` - Number of data bits in a UART frame
    /// * `parity` - Parity mode
    /// * `stop_bits` - Number of stop bits in a UART frame
    ///
    /// # Examples
    /// ```
    /// use riot_wrappers::uart::{DataBits, Parity, StopBits, UartDevice};
    /// let mut received_data = 0u8;
    /// let mut uart = UartDevice::new_without_rx(uart_type_t_STM32_USART, 115200)
    /// .unwrap_or_else(|e| panic!("Error initializing UART: {e:?}"));   
    /// uart.set_mode(DataBits::Eight, Parity::None, StopBits::One)   
    /// .unwrap_or_else(|e| panic!("Error setting UART mode: {e:?}"));
    /// ```
    #[cfg(feature = "uart_set_mode")]
    pub fn set_mode(
        &mut self,
        data_bits: DataBits,
        parity: Parity,
        stop_bits: StopBits,
    ) -> Result<(), UartDeviceStatus> {
        unsafe {
            match UartDeviceStatus::from_c(uart_mode(
                self.dev,
                data_bits.to_c(),
                parity.to_c(),
                stop_bits.to_c(),
            )) {
                UartDeviceStatus::Success => Ok(()),
                status => Err(status),
            }
        }
    }

    /// Transmits the given data via the `UART`-device
    ///
    /// # Examples
    /// ```
    /// use riot_wrappers::uart::UartDevice;
    /// let mut uart = UartDevice::new_without_rx(uart_type_t_STM32_USART, 115200)
    ///    .unwrap_or_else(|e| panic!("Error initializing UART: {e:?}"));
    /// uart.write(b"Hello from UART\n");
    pub fn write(&mut self, data: &[u8]) {
        unsafe {
            uart_write(self.dev, data.as_ptr(), data.len() as size_t);
        }
    }

    /// Turns on the power from the `UART-Device`
    pub fn power_on(&mut self) {
        unsafe { uart_poweron(self.dev) };
    }

    /// Turns off the power from the `UART-Device`
    pub fn power_off(&mut self) {
        unsafe { uart_poweroff(self.dev) };
    }

    /// Enables collision detection check
    #[cfg(riot_module_periph_uart_collision)]
    pub fn collision_detect_enable(&mut self) {
        unsafe { uart_collision_detect_enable(self.dev) };
    }

    /// Disables collision detection check
    #[cfg(riot_module_periph_uart_collision)]
    pub fn collision_detect_disable(&mut self) {
        unsafe { uart_collision_detect_disable(self.dev) };
    }

    /// Disables collision detection and returns if a collision occurred during last transfer
    #[cfg(riot_module_periph_uart_collision)]
    pub fn collision_detected(&mut self) -> bool {
        unsafe { uart_collision_detected(self.dev) }
    }

    /// This function normally does not need to be called. But in some case, the pins on the `UART`
    /// might be shared with some other functionality (like `GPIO`). In this case, it is necessary
    /// to give the user the possibility to init the pins again.
    #[cfg(riot_module_periph_uart_reconfigure)]
    pub unsafe fn init_pins(&mut self) {
        uart_init_pins(self.dev);
    }

    /// Change the pins back to plain GPIO functionality
    #[cfg(riot_module_periph_uart_reconfigure)]
    pub unsafe fn deinit_pins(&mut self) {
        uart_deinit_pins(self.dev);
    }

    /// Get the RX pin
    #[cfg(riot_module_periph_uart_reconfigure)]
    pub fn get_pin_rx(&mut self) -> gpio_t {
        unsafe { uart_pin_rx(self.dev) }
    }

    /// Get the TX pin
    #[cfg(riot_module_periph_uart_reconfigure)]
    pub fn get_pin_tx(&mut self) -> gpio_t {
        unsafe { uart_pin_tx(self.dev) }
    }

    /// Configure the function that will be called when a start condition is detected
    /// This will not enable / disable the generation of the RX start interrupt
    /// # Arguments
    /// * `user_fxopt` - The user defined callback function that gets called when a start condition is detected
    #[cfg(riot_module_periph_uart_rxstart_irq)]
    pub fn rxstart_irq_configure<F>(&mut self, user_fxopt: &'a mut F)
    where
        F: FnMut(),
    {
        unsafe {
            uart_rxstart_irq_configure(
                dev,
                Self::rxstart_callback::<F>,
                user_fxopt as *mut _ as *mut c_void,
            )
        };
    }

    /// Enable the RX start interrupt
    #[cfg(riot_module_periph_uart_rxstart_irq)]
    pub fn rxstart_irq_enable(&mut self) {
        unsafe { uart_rxstart_irq_enable(self.dev) };
    }

    /// Disable the RX start interrupt
    #[cfg(riot_module_periph_uart_rxstart_irq)]
    pub fn rxstart_irq_disable(&mut self) {
        unsafe { uart_rxstart_irq_disable(self.dev) };
    }

    /// This is the callback that gets called directly from the kernel if new data from the `UART` is received
    /// # Arguments
    /// * `user_callback` - The address pointing to the user defined callback
    /// * `data` - The newly received data from the `UART`  
    unsafe extern "C" fn new_data_callback<F>(user_callback: *mut c_void, data: u8)
    where
        F: FnMut(u8),
    {
        (*(user_callback as *mut F))(data); //We cast the void* back to the closure and call it
    }

    /// This is the callback that gets called directly from the kernel when a start condition is detected
    /// # Arguments
    /// * `user_callback` - The address pointing to the user defined callback
    #[cfg(riot_module_periph_uart_rxstart_irq)]
    unsafe extern "C" fn rxstart_callback<F>(user_callback: *mut c_void)
    where
        F: FnMut(),
    {
        (*(user_callback as *mut F))(); //We cast the void* back to the closure and call it
    }
}

impl<'a> Drop for UartDevice<'a> {
    /// The `drop` method resets the `UART`, removes the interrupt and tries
    /// to reset the `GPIO` pins if possible
    fn drop(&mut self) {
        unsafe {
            uart_init(self.dev, 9600, None, ptr::null_mut());
            #[cfg(riot_module_periph_uart_reconfigure)]
            self.deinit_pins();
        }
    }
}
