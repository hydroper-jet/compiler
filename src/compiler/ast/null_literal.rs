use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct NullLiteral {
    pub location: Location,
}

impl NullLiteral {
    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier, context: &ExpressionVerifyContext) -> Result<Option<Symbol>, DeferVerificationError> {
        if let Some(t) = context.context_type.as_ref() {
            if t.is_nullable_type() {
                return Ok(Some(verifier.host.factory().create_null_constant(t)));
            }
        }
        Ok(Some(verifier.host.factory().create_null_constant(&verifier.host.any_type())))
    }
}