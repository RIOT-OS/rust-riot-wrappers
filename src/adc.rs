pub struct ADCLine(riot_sys::adc_t);

impl ADCLine {
    /// Initialize an ADC line and get it as a handle. This is declared as unsafe as it may only
    /// be called once. (A safe abstraction would need to check which RIOT devices have been
    /// initialized already).
    pub unsafe fn init(line: riot_sys::adc_t) -> Result<Self, i32> {
        let success = unsafe { riot_sys::adc_init(line) };
        match success {
            0 => Ok(ADCLine(line)),
            e => Err(e),
        }
    }
}

/// A configured representation of the single operating-system level ADC that RIOT exposes via its
/// ADC API. The individual ADC lines are addressed as ADCLine structs and can be used uniformly
/// with the (any) ADC struct. The differenes between the hardware ADCs are as hidden to the
/// embedded_hal API as they are hidden to RIOT applications.
pub struct ADC {
    pub resolution: riot_sys::adc_res_t,
}

impl embedded_hal::adc::Channel<ADC> for ADCLine {
    type ID = riot_sys::adc_t;
    fn channel() -> Self::ID {
        unimplemented!("See https://github.com/rust-embedded/embedded-hal/issues/110")
    }
}

impl embedded_hal::adc::OneShot<ADC, i32, ADCLine> for ADC {
    type Error = !;

    fn read(&mut self, pin: &mut ADCLine) -> nb::Result<i32, !> {
        // Sorry, blocking still
        Ok(unsafe { riot_sys::adc_sample(pin.0, self.resolution) })
    }
}
