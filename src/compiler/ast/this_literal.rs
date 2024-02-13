use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ThisLiteral {
    pub location: Location,
}

impl ThisLiteral {
    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier) -> Result<Option<Symbol>, DeferVerificationError> {
        let activation = verifier.scope.find_activation();
        if activation.is_some() && activation.as_ref().unwrap().this().is_some() {
            Ok(activation.clone().unwrap().this())
        } else {
            verifier.add_verify_error(&self.location, DiagnosticKind::IllegalThisReference, diagnostic_arguments![]);
            Ok(None)
        }
    }
}