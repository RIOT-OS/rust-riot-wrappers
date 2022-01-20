/// Workaround to get name for Never type for the few places it is actually needed (eg. as return
/// type for callback signatures)
///
/// From <https://github.com/rust-lang/rust/issues/43301#issuecomment-912390203>, adjusted for
/// usability with pub interfaces by using a pub trait in a private module (sealing).

pub trait NeverHelper {
    type Never;
}
impl<T> NeverHelper for fn() -> T {
    type Never = T;
}
pub(crate) type Never = <fn() -> ! as NeverHelper>::Never;
