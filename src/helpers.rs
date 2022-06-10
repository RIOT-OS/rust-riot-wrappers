//! Small tools used in different wrappers without being actually public

/// Generalization of the Never type extracting workaround from
/// <https://github.com/rust-lang/rust/issues/43301#issuecomment-912390203> -- also useful to
/// extract return types of functions that (in what is compatible behavior in C) change their
/// return types.
///
/// With the introduction of ArgXType, it's actually not a ReturnType but AnyFunctionInvolvedType
/// extractor any more...
pub trait ReturnTypeExtractor {
    type ReturnType;
    type Arg1Type;
    type Arg2Type;
}
impl<T> ReturnTypeExtractor for fn() -> T {
    type ReturnType = T;
    type Arg1Type = ();
    type Arg2Type = ();
}
impl<T, I1> ReturnTypeExtractor for Option<unsafe extern "C" fn(I1) -> T> {
    type ReturnType = T;
    type Arg1Type = I1;
    type Arg2Type = ();
}
impl<T, I1, I2> ReturnTypeExtractor for Option<unsafe extern "C" fn(I1, I2) -> T> {
    type ReturnType = T;
    type Arg1Type = I1;
    type Arg2Type = I2;
}
