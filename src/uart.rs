#![allow(dead_code)]

use core::{mem, ptr};
use riot_sys::libc::{c_int, c_uint, c_void};
use riot_sys::*;

/// This struct contains the `UART` device and handles all operation regarding it
///
/// [UART implementation]: https://doc.riot-os.org/group__drivers__periph__uart.html
#[derive(Debug)]
pub struct UartDevice {
    dev: uart_t,
}

/// This enum representatives the status returned by various `UART`-functions
#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
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


#[derive(Debug, Eq, PartialEq)]
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


impl UartDevice {
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
    /// let mut received_data = 0u8;
    /// let mut uart = UartDevice::new(uart_type_t_STM32_USART, 115200, &mut |data| {
    ///     received_data = data;
    /// })
    /// .unwrap_or_else(|e| panic!("Error initializing UART: {e:?}"));
    /// uart.write(b"Hello from UART");
    /// ```
    pub fn new<F>(dev: uart_t, baud: u32, user_callback: &mut F) -> Result<Self, UartDeviceStatus>
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
                UartDeviceStatus::Success => Ok(Self { dev }),
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
    /// let mut received_data = 0u8;
    /// let mut uart = UartDevice::new_without_rx(uart_type_t_STM32_USART, 115200)
    /// .unwrap_or_else(|e| panic!("Error initializing UART: {e:?}"));
    /// uart.write(b"Hello from UART");
    /// ```
    pub fn new_without_rx(dev: uart_t, baud: u32) -> Result<Self, UartDeviceStatus> {
        unsafe {
            match UartDeviceStatus::from_c(uart_init(dev, baud, None, ptr::null_mut())) {
                UartDeviceStatus::Success => Ok(Self { dev }),
                status => Err(status),
            }
        }
    }


    /// Sets the mode according to the given parameters
    /// Should the parameters be invalid, the function returns a Err<UartDeviceStatus>
    /// # Arguments
    /// * `data_bits` - Number of data bits in a UART frame
    /// * `parity` - Parity mode
    /// * `stop_bits` - Number of stop bits in a UART frame
    ///
    /// # Examples
    /// ```
    /// let mut received_data = 0u8;
    /// let mut uart = UartDevice::new_without_rx(uart_type_t_STM32_USART, 115200)
    /// .unwrap_or_else(|e| panic!("Error initializing UART: {e:?}"));   
    /// uart.set_mode(DataBits::Eight, Parity::None, StopBits::One)   
    /// .unwrap_or_else(|e| panic!("Error setting UART mode: {e:?}"));
    /// ```
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
    /// let mut uart = UartDevice::new_without_rx(uart_type_t_STM32_USART, 115200)
    ///    .unwrap_or_else(|e| panic!("Error initializing UART: {e:?}"));
    /// uart.write(b"Hello from UART\n");
    pub fn write(&mut self, data: &[u8]) {
        unsafe {
            uart_write(self.dev, data.as_ptr(), data.len() as size_t);
        }
    }

    /// The function turns off the power from the `UART-Device`
    pub fn power_off(&mut self) {
        unsafe { uart_poweroff(self.dev) };
    }

    /// The function turns on the power from the `UART-Device`
    pub fn power_on(&mut self) {
        unsafe { uart_poweron(self.dev) };
    }

    /// Disables collision detection check of the given UART device
    #[cfg(riot_module_periph_uart_collision)]
    pub fn collision_detect_disable(&mut self) {
        unsafe { uart_collision_detect_disable(self.dev) };
    }

    /// Enables collision detection check of the given UART device
    #[cfg(riot_module_periph_uart_collision)]
    pub fn collision_detect_enable(&mut self) {
        unsafe { uart_collision_detect_enable(self.dev) };
    }

    /// Disables collision detection of the given UART device and return true if collision occurred during last transfer
    #[cfg(riot_module_periph_uart_rxstart_irq)]
    pub fn collision_detected(&mut self) -> bool {
        unsafe { uart_collision_detected(self.dev) }
    }

    /// Change the pins of the given UART back to plain GPIO functionality. It also consumes the `UART`, so it cannot
    /// be used afterwards
    #[cfg(riot_module_periph_uart_reconfigure)]
    pub fn deinit_pins(self) {
        unsafe { uart_deinit_pins(self.dev) };
    }

    /// After calling uart_init, the pins must be initialized (i.e. uart_init is calling this function internally).
    /// In normal cases, this function will not be used. But there are some devices, that use UART bus lines also
    /// for other purposes and need the option to dynamically re-configure one or more of the used pins. So
    /// they can take control over certain pins and return control back to the UART driver using this function.
    #[cfg(riot_module_periph_uart_reconfigure)]
    pub fn init_pins(&mut self) {
        unsafe { uart_init_pins(self.dev) };
    }

    /// Get the RX pin of the given UART device
    #[cfg(riot_module_periph_uart_reconfigure)]
    pub fn get_pin_rx(&mut self) -> gpio_t {
        unsafe { uart_pin_rx(self.dev) }
    }

    /// Get the TX pin of the given UART device
    #[cfg(riot_module_periph_uart_reconfigure)]
    pub fn get_pin_tx(&mut self) -> gpio_t {
        unsafe { uart_pin_tx(self.dev) }
    }

    /// Configure the function that will be called when a start condition is detected
    ///
    /// # Arguments
    /// * `user_fxopt` - The user defined callback function called when a start condition is detected
    #[cfg(riot_module_periph_uart_rxstart_irq)]
    pub fn rxstart_irq_configure<F>(&mut self, user_fxopt: &mut F)
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
    #[cfg(riot_module_riot_module_periph_uart_rxstart_irq)]
    pub fn rxstart_irq_enable(&mut self) {
        unsafe { uart_rxstart_irq_enable(self.dev) };
    }

    /// Disable the RX start interrupt
    #[cfg(riot_module_riot_module_periph_uart_rxstart_irq)]
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
    /// * `data` - The newly received data from the `UART`  
    #[cfg(riot_module_periph_uart_rxstart_irq)]
    unsafe extern "C" fn rxstart_callback<F>(user_callback: *mut c_void)
    where
        F: FnMut(),
    {
        (*(user_callback as *mut F))();
    }
}

impl Drop for UartDevice {
    /// The `drop` method resets the uart to 9600 baud and removes the user defined callback
    /// # Safety
    /// At this moment it is unclear if this implementation is the right way to go. There might
    /// be a better solution...
    /// Also if the build is in debug mode and the `UART` is reinitialized unsuccessfully, the code panics which
    /// is definitely <b>NOT</b> the right behavior at this point!
    fn drop(&mut self) {
        if cfg!(riot_module_periph_uart_reconfigure) {
            #[cfg(riot_module_periph_uart_reconfigure)]
            deinit_pins(self); //TODO Check if this also removes the irq
        } else {
            unsafe {
                let status =
                    UartDeviceStatus::from_c(uart_init(self.dev, 9600, None, ptr::null_mut()));
                debug_assert_eq!(
                    status,
                    UartDeviceStatus::Success,
                    "Error while deinitializing UART: {status:?}"
                );
                self.power_off();
            }
        }
    }
}
