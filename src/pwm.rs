use crate::println;
use embedded_hal::Pwm;
use riot_sys::pwm_t;

/// PWM modes
///
/// [`PWMMode::CENTER`] exists in RIOT but is marked as "not supported"
#[derive(Debug)]
pub enum PWMMode {
    LEFT,
    RIGHT,
    CENTER,
}

impl PWMMode {
    /// Converts the rust enum to a c type
    pub fn to_c(self) -> riot_sys::pwm_mode_t {
        match self {
            Self::LEFT => riot_sys::pwm_mode_t_PWM_LEFT,
            Self::RIGHT => riot_sys::pwm_mode_t_PWM_RIGHT,
            Self::CENTER => riot_sys::pwm_mode_t_PWM_CENTER,
        }
    }
}

#[derive(Debug)]
pub struct PWMDevice<const CHANNELS: usize> {
    dev: pwm_t,
    channel_duty_values: [u16; CHANNELS], //Needed because of embedded_hal implementation
    frequency: u32,                       //Needed because of embedded_hal implementation
}

pub type Hz = u32;

impl<const CHANNELS: usize> PWMDevice<CHANNELS> {
    /// Creates and initializes a [`PWMDevice`] with the given frequency in Hz and the resolution of a period/duty cycle
    ///
    /// If the given frequency and resolution values can not be acchieved on the current device, the
    /// resolution will be kept the same while the frequency will be lowered until the device can handle the combination.
    /// The actual set frequency can the be obtained with [`PWMDevice::get_frequency`].
    ///
    /// Returns the initialized [`PWMDevice`] on success
    ///
    /// **Note** that the `CHANNELS` Variable is only used to cache the duty values for each of the `CHANNELS` number of
    /// pwm channels. This is used by [`embedded_hal::Pwm::get_duty`]. If this is not needed setting `CHANNELS` to `0` saves memory.
    pub fn new(dev: pwm_t, mode: PWMMode, frequency: Hz, resolution: u16) -> Result<Self, ()> {
        let mut pwm_dev = PWMDevice {
            dev,
            channel_duty_values: [0; CHANNELS],
            frequency,
        };

        match pwm_dev.init(mode, frequency, resolution) {
            Ok(()) => Ok(pwm_dev),
            // Sadly RIOTs interface does not allow for a more elaborate error handling.
            // Maybe something like errno is set without the doc telling us, which could be used here.
            Err(()) => Err(()),
        }
    }

    /// Creates a PWM device from an already initialized c type.
    ///
    /// **Note** that the `CHANNELS` Variable is only used to cache the duty values for each of the `CHANNELS` number of
    /// pwm channels. This is used by [`embedded_hal::Pwm::get_duty`]. If this is not needed setting `CHANNELS` to `0` saves memory.
    ///
    /// ## Important:
    /// It is **important** to make sure that the provided **device is already initialized** by using [`riot_sys::pwm_init`](https://rustdoc.etonomy.org/riot_sys/fn.pwm_init.html).
    /// Using the returned device otherwise results in **undefined behavior**!
    ///
    /// Also note, that the given frequency is only to be used by [`embedded_hal::Pwm::get_period`] or [`PWMDevice::get_frequency`].
    /// Just setting this to `x` will only result in those two functions returning `x` but will have no other impact, if
    /// for example the given [`pwm_t`] is initialized by a board and the actually set frequency is unknown.
    ///
    pub unsafe fn new_without_init(dev: pwm_t, frequency: Hz) -> Self {
        PWMDevice {
            dev,
            channel_duty_values: [0; CHANNELS],
            frequency,
        }
    }

    /// Initializes the [`PWMDevice`] with the given frequency in Hz and resolution of a period/duty cycle.
    ///
    /// If the given frequency and resolution values can not be acchieved on the current device, the
    /// resolution will be kept the same while the frequency will be lowered until the device can handle the combination.
    /// The resulting frequency is written into the [`PWMDevice`].
    ///
    /// Uses a [`Result`] in anticipation of a more usefull error handling in the future.
    ///
    /// Returns if the initialization was a success
    fn init(&mut self, mode: PWMMode, frequency: Hz, resolution: u16) -> Result<(), ()> {
        let err = unsafe { riot_sys::pwm_init(self.dev, mode.to_c(), frequency, resolution) };

        match err {
            0 => Err(()),
            freq => {
                // Set frequency in the device
                self.frequency = freq;
                Ok(())
            }
        }
    }

    /// Returns the number of available channels for this device
    pub fn channels(&self) -> u8 {
        unsafe { riot_sys::pwm_channels(self.dev).min(CHANNELS as u8) }
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
    /// value: `0: 0%, u16::MAX: 100%` duty_cycle
    pub fn set(&mut self, channel: u8, value: u16) {
        unsafe {
            riot_sys::pwm_set(self.dev, channel, value);
        }

        let channel = channel as usize;
        // Ignore if entry does not exists because
        // only embedded_hal interface cares about those values
        // and the implementation already checks for this
        if channel < (CHANNELS) {
            self.channel_duty_values[channel] = value;
        }
    }

    /// Returns the used frequency of this [`PWMDevice`] in Hz
    ///
    pub fn get_frequency(&self) -> u32 {
        self.frequency
    }
}

pub type Seconds = f32;

impl<const CHANNELS: usize> Pwm for PWMDevice<CHANNELS> {
    type Channel = u8;
    type Duty = u16;
    type Time = Seconds;

    fn disable(&mut self, _channel: Self::Channel) {
        println!("[ERROR] RIOT does not support enabling/disabling single channels")
    }

    fn enable(&mut self, _channel: Self::Channel) {
        println!("[ERROR] RIOT does not support enabling/disabling single channels")
    }

    fn get_period(&self) -> Self::Time {
        1. / self.frequency as f32
    }

    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        self.channel_duty_values[channel as usize]
    }

    fn get_max_duty(&self) -> Self::Duty {
        u16::MAX
    }

    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        if (channel as usize) >= CHANNELS {
            panic!("Tried to set duty for non existing channel: {channel}")
        }

        self.set(channel, duty);
    }

    fn set_period<P>(&mut self, _period: P)
    where
        P: Into<Self::Time>,
    {
        println!("RIOT does not support setting the period after initialisation")
    }
}
