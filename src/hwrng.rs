use embedded_hal::blocking::rng::Read;

#[derive(Debug)]
#[non_exhaustive]
pub enum HWRNGError {
    Other,
}

/// Represents RIOTs hwrng module. It can be used via
/// `embedded_hal`s [`embedded_hal::blocking::rng::Read`] trait.
///
/// The main purpose of this module is to generate seeds for PRNGs like
/// [`rand::rngs::StdRng`] or [`crate::random::Random`] (see `prng` module).
///
/// # Security
/// As stated in RIOTs hwrng module-description the quality of the generated
/// random data may vary drastically between boards. If you want to use this
/// for e.g. cryptography make sure your current boards hwrng implementation
/// provides random data with sufficient randomness.
#[derive(Debug)]
pub struct HWRNG;

impl Read for HWRNG {
    type Error = HWRNGError;

    fn read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        unsafe {
            riot_sys::hwrng_read(buffer.as_mut_ptr() as *mut _, buffer.len() as u32);
        }
        Ok(())
    }
}
