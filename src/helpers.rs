//! Small tools used in different wrappers without being actually public

/// Generalization of the Never type extracting workaround from
/// <https://github.com/rust-lang/rust/issues/43301#issuecomment-912390203> -- also useful to
/// extract return types of functions that (in what is compatible behavior in C) change their
/// return types.
pub trait ReturnTypeExtractor {
    type ReturnType;
}
impl<T> ReturnTypeExtractor for fn() -> T {
    type ReturnType = T;
}
impl<T, I1> ReturnTypeExtractor for Option<unsafe extern "C" fn(I1) -> T> {
    type ReturnType = T;
}
