/// Workaround to get name for Never type for the few places it is actually needed (eg. as return
/// type for callback signatures)
///
/// From <https://github.com/rust-lang/rust/issues/43301#issuecomment-912390203>, adjusted for
/// usability with pub interfaces by using a pub trait in a private module (sealing).

pub trait WithOutput {
    type Output;
}
impl<T> WithOutput for fn() -> T {
    type Output = T;
}
pub(crate) type Never = <fn() -> ! as WithOutput>::Output;
