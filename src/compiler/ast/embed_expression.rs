use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

/// The `embed {...}` expression.
/// 
/// It is semantically assigned an `EmbedValue` symbol.
#[derive(Clone, Serialize, Deserialize)]
pub struct EmbedExpression {
    pub location: Location,
    pub description: ObjectInitializer,
}

impl EmbedExpression {
    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier, exp: &Rc<Expression>, context_type: Option<Symbol>) -> Result<Option<Symbol>, DeferVerificationError> {
        ()
    }
}