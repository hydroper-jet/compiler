use crate::ns::*;

#[derive(Clone, Debug)]
pub struct DeferVerificationError;

#[derive(Clone, Debug)]
pub struct ExpectedTypeError;

#[derive(Clone, Debug)]
pub enum PropertyResolutionError {
    AmbiguousReference { name: String },
    DeferVerification,
    VoidBase,
    NullableBase { nullable_type: Symbol },
}

#[derive(Clone)]
pub enum MethodOverridingError {
    DeferVerification,
    MustOverrideAMethod,
    CannotOverrideTypeParameterizedMethod,
    CannotIntroduceTypeParameters,
    IncompatibleSignature {
        expected_signature: Symbol,
        actual_signature: Symbol,
    },
    OverridingFinalMethod,
}