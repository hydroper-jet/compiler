use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct QualifiedIdentifier {
    pub location: Location,
    pub attribute: bool,
    pub qualifier: Option<Rc<Expression>>,
    pub id: QualifiedIdentifierIdentifier,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum QualifiedIdentifierIdentifier {
    Id((String, Location)),
    Brackets(Rc<Expression>),
}

impl QualifiedIdentifier {
    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier) -> Result<Option<(Option<Symbol>, SemanticPropertyKey)>, DeferVerificationError> {
        let QualifiedIdentifier {
            qualifier,
            id,
            ..
        } = self;

        let mut failed = false;

        let mut result_qual: Option<Symbol> = None;
        if let Some(qualifier) = qualifier {
            result_qual = verifier.limit_expression_type(qualifier, &verifier.host.namespace_type())?;
            if result_qual.is_none() {
                failed = true;
            }
        }

        let mut result_key: Option<SemanticPropertyKey> = None;

        match id {
            QualifiedIdentifierIdentifier::Id((id, _)) => {
                result_key = Some(SemanticPropertyKey::String(id.clone()));
            },
            QualifiedIdentifierIdentifier::Brackets(exp) => {
                let v = verifier.limit_expression_type(exp, &verifier.host.string_type())?;
                if let Some(v) = v {
                    result_key = Some(SemanticPropertyKey::Value(v));
                } else {
                    failed = true;
                }
            },
        }

        if failed {
            Ok(None)
        } else {
            Ok(Some((result_qual, result_key.unwrap())))
        }
    }

    pub(crate) fn verify_as_exp(&self, verifier: &mut VerifierVerifier, exp: &Rc<Expression>, followed_by_type_arguments: bool) -> Result<Option<Symbol>, DeferVerificationError> {
        let qn = self.verify(verifier)?;
        if qn.is_none() {
            return Ok(None);
        }
        let (qual, key) = qn.unwrap();
        //
    }
}