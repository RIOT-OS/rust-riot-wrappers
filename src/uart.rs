#![allow(dead_code)]

use core::ptr;
use riot_sys::libc::{c_int, c_void};
use riot_sys::*;

/// This struct contains the `UART` device and handles all operation regarding it
pub struct UartDevice {
    dev: uart_t,
}

/// This enum representatives the status returned by various `UART`-functions
#[derive(Debug, Eq, PartialEq)]
pub enum UartDeviceStatus {
    Success,
    InvalidDevice,
    UnsupportedConfig,
    Other,
}

impl UartDeviceStatus {
    /// Converts the given `c_int` into the matching Enum representation
    fn from_c_int(n: c_int) -> Self {
        const _ENODEV: c_int = -(ENODEV as c_int);
        const _ENOTSUP: c_int = -(ENOTSUP as c_int);
        match n {
            0 => Self::Success,
            _ENODEV => Self::InvalidDevice,
            _ENOTSUP => Self::UnsupportedConfig,
            _ => Self::Other,
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
    /// * `user_callback` The user defined callback that gets called from the os whenever new data is received from the `UART`. More about it is described below
    ///
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// We have to cast the given closure reference into a `void*`. This is called `type-erasure` and
    /// it is necessary because `C` has no build in functions for such complex mechanisms.
    /// We also give the callback the generic parameter `F` which contains the unique signature
    /// of the closure. This is very imported because only with this we can cast it back later
    /// (as described below).
    ///
    /// Also we check if the device is initialized successfully and return a rust-idiomatic
    /// Result as described above
    ///
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
        F: FnMut(u8), //FnMut(u8) is a closure that can capture the environment. It has one argument of type u8 which will be the received data from the UART
    {
        unsafe {
            match UartDeviceStatus::from_c_int(uart_init(
                dev,
                baud,
                Some(Self::new_data_callback::<F>), // Here we pass in our callback
                user_callback as *mut _ as *mut c_void, // Here we erase the type of the closure by casting it's reference into a void*
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
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it is safe because we check if the device is initialized successfully and
    /// return a rust-idiomatic Result as described above
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
            match UartDeviceStatus::from_c_int(uart_init(dev, baud, None, ptr::null_mut())) {
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
    /// uart.set_mode(
    ///     riot_sys::uart_data_bits_t_UART_DATA_BITS_8,
    ///     riot_sys::uart_parity_t_UART_PARITY_NONE,
    ///     uart_stop_bits_t_UART_STOP_BITS_1,
    /// ).unwrap_or_else(|e| panic!("Error setting UART mode: {e:?}"));
    /// ```
    pub fn set_mode(
        &mut self,
        data_bits: uart_data_bits_t,
        parity: uart_parity_t,
        stop_bits: uart_stop_bits_t,
    ) -> Result<(), UartDeviceStatus> {
        unsafe {
            match UartDeviceStatus::from_c_int(uart_mode(self.dev, data_bits, parity, stop_bits)) {
                UartDeviceStatus::Success => Ok(()),
                status => Err(status),
            }
        }
    }

    /// Transmits the given data via the `UART`-device
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it is safe because the rust compiler guarantees, that the slice pointing to the
    /// data is always valid
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
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it safe because we pass no parameters to the function
    pub fn power_off(&mut self) {
        unsafe { uart_poweroff(self.dev) };
    }

    /// The function turns on the power from the `UART-Device`
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it safe because we pass no parameters to the function
    pub fn power_on(&mut self) {
        unsafe { uart_poweron(self.dev) };
    }

    /// Disables collision detection check of the given UART device
    ///
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it's safe because we pass no parameters to the function
    #[cfg(riot_module_periph_uart_rxstart_irq)]
    pub fn collision_detect_disable(&mut self) {
        unsafe { uart_collision_detect_disable(self.dev) };
    }

    /// Enables collision detection check of the given UART device
    ///
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it's safe because we pass no parameters to the function
    #[cfg(riot_module_periph_uart_rxstart_irq)]
    pub fn collision_detect_enable(&mut self) {
        unsafe { uart_collision_detect_enable(self.dev) };
    }

    /// Disables collision detection of the given UART device
    /// and return true if collision occurred during last transfer
    ///
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it's safe because we pass no parameters to the function
    #[cfg(riot_module_periph_uart_rxstart_irq)]
    pub fn collision_detected(&mut self) -> bool {
        unsafe { uart_collision_detected(self.dev) }
    }

    /// Change the pins of the given UART back to plain GPIO functionality.
    ///
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it's safe because we pass no parameters to the function
    #[cfg(riot_module_periph_uart_reconfigure)]
    pub fn deinit_pins(&mut self) {
        unsafe { uart_deinit_pins(self.dev) };
    }


    /// Get the RX pin of the given UART device
    ///
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it's safe because we pass no parameters to the function
    #[cfg(riot_module_periph_uart_reconfigure)]
    pub fn get_pin_rx(&mut self) {
        unsafe { uart_pin_rx(self.dev) };
    }

    /// Get the TX pin of the given UART device
    ///
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it's safe because we pass no parameters to the function
    #[cfg(riot_module_periph_uart_reconfigure)]
    pub fn get_pin_tx(&mut self) {
        unsafe { uart_pin_tx(self.dev) };
    }

    /// Configure the function that will be called when a start condition is detected.
    ///
    /// # Arguments
    /// * `dev` - The uart_t handle to the hardware device
    /// * `user_fxopt` - The user defined callback function called when a start condition is detected
    ///
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it's safe because we pass no parameters to the function
    #[cfg(riot_module_periph_uart_rxstart_irq)]
    pub fn rxstart_irq_configure<F>(dev: uart_t, user_fxopt: &mut F)
    where
        F: FnMut(u8),
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
    ///
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it's safe because we pass no parameters to the function
    #[cfg(riot_module_riot_module_periph_uart_rxstart_irq)]
    pub fn rxstart_irq_enable(&mut self) {
        unsafe { uart_rxstart_irq_enable(self.dev) };
    }

    /// Disable the RX start interrupt
    ///
    /// # Safety
    /// The unsafe here is necessary because we call a `C`-function which is inherently unsafe.
    /// In this case it's safe because we pass no parameters to the function
    #[cfg(riot_module_riot_module_periph_uart_rxstart_irq)]
    pub fn rxstart_irq_disable(&mut self) {
        unsafe { uart_rxstart_irq_disable(self.dev) };
    }

    /// This is the callback that gets called directly from the kernel
    /// # Safety
    /// It is safe for us to cast the void* back to the user-defined closure,
    /// because it's signature type `F` gets passed to us from the callers side. Therefore, the
    /// signature and the captured variables are pointing to the exact same addresses
    /// from the stack-frame that owns the device
    ///
    /// # Arguments
    /// * `user_callback` - The address pointing to the user defined callback
    /// * `data` - The newly received data from the `UART`  
    unsafe extern "C" fn new_data_callback<F>(user_callback: *mut c_void, data: u8)
    where
        F: FnMut(u8),
    {
        (*(user_callback as *mut F))(data); //We cast the void* back to the closure and call it
    }

    unsafe extern "C" fn rxstart_callback<F>(user_callback: *mut c_void)
    where
        F: FnMut(u8),
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
        unsafe {
            let status =
                UartDeviceStatus::from_c_int(uart_init(self.dev, 9600, None, ptr::null_mut()));
            debug_assert_eq!(
                status,
                UartDeviceStatus::Success,
                "Error while deinitializing UART: {status:?}"
            );
            self.power_off();
        }
    }
}
