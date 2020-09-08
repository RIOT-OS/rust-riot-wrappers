//! Common error handling components for the RIOT operating system

use core::convert::TryInto;

pub trait NegativeErrorExt {
    type Out;

    /// Convert to a Result that is successful if the input value is zero or positive, or a
    /// NumericError if it is negative
    fn negative_to_error(self) -> Result<Self::Out, NumericError>;
}

/// An error that is expressed as a negative number
///
/// Ideally, that constraint should be expressed in the type system to allow the compiler to
/// represent `Result<positive_usize, NumericError>` as just the isize it originally was. For the
/// time being, this works well enough, and performance evaluation can later be done against a
/// manually implemented newtype around isize that'd be used to represent the Result.
#[derive(Debug)]
pub struct NumericError {
    pub number: isize,
}

impl NumericError {
    /// Construct a NumericError from a riot_sys constant
    ///
    /// ```
    /// let err = NumericError::from(riot_sys::ENOTSUP);
    /// println!("{}", err); # NumericError { number: -61 }
    /// ```
    pub fn from(name: isize) -> Self {
        NumericError { number: -name }
    }

    pub fn again_is_wouldblock(self) -> nb::Error<Self> {
        match -self.number as u32 {
            riot_sys::EAGAIN => nb::Error::WouldBlock,
            _ => nb::Error::Other(self)
        }
    }
}

// Would be nice, but there's no strerror
//
// impl core::fmt::Display for NumericError {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
//         write!(f, "Error {} ({})", self.number, ...)
//     }
// }

impl<T> NegativeErrorExt for T where
    T: num_traits::Zero + core::cmp::PartialOrd + TryInto<isize>,
{
    type Out = T;

    fn negative_to_error(self) -> Result<Self::Out, NumericError> {
        if self >= Self::zero() {
            Ok(self)
        } else {
            Err(NumericError { number: self.try_into().unwrap_or(-(riot_sys::EOVERFLOW as isize)) })
        }
    }
}
