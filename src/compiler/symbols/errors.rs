#[derive(Clone, Debug)]
pub struct DeferVerificationError;

#[derive(Clone, Debug)]
pub struct ExpectedTypeError;

#[derive(Clone, Debug)]
pub enum PropertyResolutionError {
    AmbiguousReference,
    DeferVerification,
}