/// Workaround to get name for Never type for the few places it is actually needed (eg. as return
/// type for callback signatures)
///
/// From <https://github.com/rust-lang/rust/issues/43301#issuecomment-912390203>, adjusted for
/// usability with pub interfaces by using a pub trait in a private module (sealing).
#[cfg(not(feature = "actual_never_type"))]
use crate::helpers::*;

#[cfg(not(feature = "actual_never_type"))]
pub(crate) type Never = <fn() -> ! as ReturnTypeExtractor>::ReturnType;
#[cfg(feature = "actual_never_type")]
pub(crate) type Never = !;
