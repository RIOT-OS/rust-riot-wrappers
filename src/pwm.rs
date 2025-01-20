use embedded_hal_0_2::Pwm;
pub use fugit::{HertzU32, Rate};
use riot_sys::pwm_t;

/// PWM modes
///
/// [`PWMMode::CENTER`] exists in RIOT but is marked as "not supported"
#[derive(Debug)]
pub enum PWMMode {
    Left,
    Right,
    Center,
}

impl PWMMode {
    /// Converts the rust enum to a c type
    fn to_c(self) -> riot_sys::pwm_mode_t {
        match self {
            Self::Left => riot_sys::pwm_mode_t_PWM_LEFT,
            Self::Right => riot_sys::pwm_mode_t_PWM_RIGHT,
            Self::Center => riot_sys::pwm_mode_t_PWM_CENTER,
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum PwmInitError {
    /// No information available on the error details
    ///
    /// This is the catch-all for unknown types, and should not be handled any different than the `_` case.
    Other,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum PWMChannelError {
    /// The specified `PWMChannel` is not available
    Unavailable,
}

/// A descriptor for a PWM channel
#[derive(Debug)]
pub struct PWMChannel {
    channel: u8,
}

/// A device for generating PWM signals on multiple channels.
///
/// **Note** that the `CHANNELS` Variable is only used to cache the duty values for each of the `CHANNELS` number of
/// PWM channels. This is used by [`embedded_hal::Pwm::get_duty`]. If this is not needed setting `CHANNELS` to `0` saves memory.
#[derive(Debug)]
pub struct PWMDevice<const CHANNELS: usize> {
    dev: pwm_t,
    channel_duty_values: [u16; CHANNELS], //Needed because of embedded_hal implementation
    frequency: HertzU32,                  //Needed because of embedded_hal implementation
    resolution: u16,
}

impl<const CHANNELS: usize> PWMDevice<CHANNELS> {
    /// Creates and initializes a [`PWMDevice`] with the given frequency in Hz and the resolution of a period/duty cycle
    ///
    /// The `index` specifies the index of a boards PWM device. To see how many devices a board supports
    /// please refer to its RIOT documentation.
    ///
    /// Note that duty-values set in [`embedded_hal::Pwm::set_duty`] have to be in [0..resolution]
    /// with `value==resolution` representing `100%` duty-cycle.
    ///
    /// If the given frequency and resolution values can not be achieved on the current device, the
    /// resolution will be kept the same while the frequency will be lowered until the device can handle the combination.
    /// The actual set frequency can the be obtained with [`PWMDevice::get_frequency`].
    ///
    /// Returns the initialized [`PWMDevice`] on success
    ///
    pub fn new(
        index: usize,
        mode: PWMMode,
        frequency: HertzU32,
        resolution: u16,
    ) -> Result<Self, PwmInitError> {
        let mut pwm_dev = PWMDevice {
            dev: unsafe { riot_sys::macro_PWM_DEV(index as u32) },
            channel_duty_values: [0; CHANNELS],
            frequency,
            resolution,
        };

        pwm_dev.init(mode, frequency, resolution)?;
        Ok(pwm_dev)
    }

    /// Creates a PWM device from an already initialized c type.
    ///
    /// **Note** that the `CHANNELS` Variable is only used to cache the duty values for each of the `CHANNELS` number of
    /// PWM channels. This is used by [`embedded_hal::Pwm::get_duty`]. If this is not needed setting `CHANNELS` to `0` saves memory.
    ///
    /// ## Important:
    /// It is **important** to make sure that the provided **device is already initialized** by using [`riot_sys::pwm_init`](https://rustdoc.etonomy.org/riot_sys/fn.pwm_init.html).
    /// Using the returned device otherwise results in **undefined behavior**!
    ///
    /// Also note, that the given frequency is only to be used by [`embedded_hal::Pwm::get_period`] or [`PWMDevice::get_frequency`].
    /// Just setting this to `x` will only result in those two functions returning `x` but will have no other impact, if
    /// for example the given [`pwm_t`] is initialized by a board and the actually set frequency is unknown.
    ///
    /// The same goes for `resolution` which is only used in [`embedded_hal::Pwm::get_max_duty`].
    pub unsafe fn new_without_init(dev: pwm_t, frequency: HertzU32, resolution: u16) -> Self {
        PWMDevice {
            dev,
            channel_duty_values: [0; CHANNELS],
            frequency,
            resolution,
        }
    }

    /// Initializes the [`PWMDevice`] with the given frequency in Hz and resolution of a period/duty cycle.
    ///
    /// If the given frequency and resolution values can not be achieved on the current device, the
    /// resolution will be kept the same while the frequency will be lowered until the device can handle the combination.
    /// The resulting frequency is written into the [`PWMDevice`].
    ///
    /// Uses a [`Result`] in anticipation of a more useful error handling in the future.
    fn init(
        &mut self,
        mode: PWMMode,
        frequency: HertzU32,
        resolution: u16,
    ) -> Result<(), PwmInitError> {
        let err = unsafe { riot_sys::pwm_init(self.dev, mode.to_c(), frequency.raw(), resolution) };

        match err {
            0 => Err(PwmInitError::Other),
            freq => {
                // Set frequency in the device
                self.frequency = Rate::<u32, 1, 1>::from_raw(freq);
                Ok(())
            }
        }
    }

    /// Provides a channel-descriptor for the given channel number.
    ///
    /// Returns an error if the channel number has no corresponding PWM channel in
    /// RIOT   
    pub fn get_channel(&self, channel_num: u8) -> Result<PWMChannel, PWMChannelError> {
        if channel_num >= self.channels() {
            Err(PWMChannelError::Unavailable)
        } else {
            Ok(PWMChannel {
                channel: channel_num,
            })
        }
    }

    /// Returns the number of available channels for this device.
    ///
    /// This is completely independent from `CHANNELS` and only returns
    /// the count of channels available in RIOT
    pub fn channels(&self) -> u8 {
        unsafe { riot_sys::pwm_channels(self.dev) }
    }

    /// Stops PWM generation on this device
    pub fn power_off(&mut self) {
        unsafe { riot_sys::pwm_poweroff(self.dev) }
    }

    /// Resumes PWM generation after power_off on this device
    pub fn power_on(&mut self) {
        unsafe {
            riot_sys::pwm_poweron(self.dev);
        }
    }

    /// Sets the duty-cycle for the given channel
    ///
    /// value: `0: 0%, resolution: 100%` duty_cycle
    fn set(&mut self, channel: PWMChannel, value: u16) {
        unsafe {
            riot_sys::pwm_set(self.dev, channel.channel, value);
        }

        let channel = channel.channel as usize;
        // Ignore if entry does not exists because
        // only embedded_hal interface cares about those values
        // and the implementation already checks for this
        if channel < CHANNELS {
            self.channel_duty_values[channel] = value;
        }
    }
}

impl<const CHANNELS: usize> Pwm for PWMDevice<CHANNELS> {
    type Channel = PWMChannel;
    type Duty = u16;
    type Time = HertzU32;

    fn disable(&mut self, _channel: Self::Channel) {
        panic!("RIOT does not support enabling/disabling single channels")
    }

    fn enable(&mut self, _channel: Self::Channel) {
        panic!("RIOT does not support enabling/disabling single channels")
    }

    fn get_period(&self) -> Self::Time {
        self.frequency
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        let channel = channel.channel as usize;

        if channel >= CHANNELS {
            panic!(
                "Tried to get duty for not cached channel: {} >= CHANNELS",
                channel
            )
        }
        self.channel_duty_values[channel]
    }

    fn get_max_duty(&self) -> Self::Duty {
        self.resolution
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        self.set(channel, duty);
    }

    fn set_period<P>(&mut self, _period: P)
    where
        P: Into<Self::Time>,
    {
        panic!("RIOT does not support setting the period after initialisation")
    }
}
