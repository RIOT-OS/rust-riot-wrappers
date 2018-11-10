pub struct ADCLine(pub riot_sys::adc_t);

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
    type Error = i32;

    fn read(&mut self, pin: &mut ADCLine) -> nb::Result<i32, Self::Error> {
        let success = unsafe { riot_sys::adc_init(pin.0) };
        match success {
            0 => Ok(unsafe { riot_sys::adc_sample(pin.0, self.resolution) }),
            e => Err(nb::Error::Other(e)),
        }
    }
}
