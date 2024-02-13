use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct RegExpLiteral {
    pub location: Location,
    pub body: String,
    pub flags: String,
}

impl RegExpLiteral {
    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier) -> Result<Option<Symbol>, DeferVerificationError> {
        Ok(Some(verifier.host.factory().create_value(&verifier.host.reg_exp_type())))
    }
}