//! Common error handling components for the RIOT operating system
//!
//! ## Constants
//!
//! Several commonly used errors are provided as constants rather than requiring the use of
//! [NumericError::from_constant] for easier use. That list is not created comprehensively but
//! populated on demand. (Copying the full list would needlessly limit RIOT's ability to slim down
//! the list).

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
#[derive(Debug, PartialEq, Eq)]
pub struct NumericError {
    number: isize,
}

impl NumericError {
    /// Construct a NumericError from a [riot_sys] constant
    ///
    /// As error constants are in their unsigned positive form, this flips the argument's sign into
    /// the negative range.
    ///
    /// ```
    /// # #![no_std]
    /// # #![feature(start)]
    /// # #[start]
    /// # fn main(_argc: isize, _argv: *const *const u8) -> isize {
    /// # use riot_wrappers::error::NumericError;
    /// # use riot_wrappers::stdio::println;
    /// let err = NumericError::from_constant(riot_sys::ENOTSUP as _);
    /// println!("{:?}", err); // NumericError { number: -61 }
    /// # 0
    /// # }
    /// ```
    ///
    /// ## Panics
    ///
    /// In debug mode, this ensures that the given error is greater than zero.
    pub const fn from_constant(name: isize) -> Self {
        debug_assert!(
            name > 0,
            "Error names are expected to be positive for conversion into negative error numbers."
        );
        NumericError { number: -name }
    }

    /// Numeric value of the error
    pub const fn number(&self) -> isize {
        self.number
    }

    /// Convert the error into an [nb::Error] that is [nb::Error::WouldBlock] if the error is
    /// `-EAGAIN`, and an actual error otherwise.
    pub fn again_is_wouldblock(self) -> nb::Error<Self> {
        if self == Self::from_constant(riot_sys::EAGAIN as _) {
            return nb::Error::WouldBlock;
        }
        nb::Error::Other(self)
    }
}

// Would be nice, but there's no strerror
//
// impl core::fmt::Display for NumericError {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
//         write!(f, "Error {} ({})", self.number(), ...)
//     }
// }

impl<T> NegativeErrorExt for T
where
    T: num_traits::Zero + core::cmp::PartialOrd + TryInto<isize>,
{
    type Out = T;

    fn negative_to_error(self) -> Result<Self::Out, NumericError> {
        if self >= Self::zero() {
            Ok(self)
        } else {
            Err(NumericError {
                number: self.try_into().unwrap_or(-(riot_sys::EOVERFLOW as isize)),
            })
        }
    }
}

macro_rules! E {
    ($e:ident) => {
        #[doc = concat!("The predefined error ", stringify!($e))]
        pub const $e: NumericError = NumericError::from_constant(riot_sys::$e as _);
    };
}

// See module level comment
E!(EAGAIN);
E!(ENOMEM);
E!(ENOSPC);
E!(EOVERFLOW);
