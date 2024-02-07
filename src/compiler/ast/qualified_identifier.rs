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
        let r = verifier.scope.resolve_property(qual, key.clone(), &verifier.host);
        if r.is_err() {
            match r.unwrap_err() {
                PropertyResolutionError::AmbiguousReference { name } => {
                    verifier.add_verify_error(&exp.location(), DiagnosticKind::AmbiguousReference, diagnostic_arguments![String(name.clone())]);
                    verifier.ast_to_symbol.set(exp, None);
                    return Ok(None);
                },
                PropertyResolutionError::DeferVerification => {
                    return Err(DeferVerificationError);
                },
                PropertyResolutionError::VoidBase => {
                    verifier.add_verify_error(&exp.location(), DiagnosticKind::AccessingPropertyOfVoidBase, diagnostic_arguments![]);
                    verifier.ast_to_symbol.set(exp, None);
                    return Ok(None);
                },
                PropertyResolutionError::NullableBase { nullable_type } => {
                    verifier.add_verify_error(&exp.location(), DiagnosticKind::AccessingPropertyOfNullableBase, diagnostic_arguments![Symbol(nullable_type)]);
                    verifier.ast_to_symbol.set(exp, None);
                    return Ok(None);
                },
            }
        }
        let r = r.unwrap();
        if r.is_none() {
            verifier.ast_to_symbol.set(exp, None);
            return Ok(None);
        }
        let r = r.unwrap();

        if !r.property_is_visible(&verifier.scope, &verifier.host) {
            verifier.add_verify_error(&exp.location(), DiagnosticKind::InaccessibleProperty, diagnostic_arguments![String(key.string_value().unwrap())]);
        }

        if r.is_reference_value() && (r.is_static_reference_value() || r.is_instance_reference_value() || r.is_scope_reference_value() || r.is_package_reference_value()) {
            let p = r.property();

            // Require type arguments
            if (p.is_origin_function() || p.is_origin_class_type() || p.is_origin_interface_type()) && p.type_parameters().is_some() && !followed_by_type_arguments {
                verifier.add_verify_error(&exp.location(), DiagnosticKind::TypeParameterizedPropertyMustBeArgumented, diagnostic_arguments![]);
            }

            // Compile-time constant
            if p.is_origin_variable_property() && p.read_only(&verifier.host) && p.constant_initializer().is_some() {
                let r = p.constant_initializer().unwrap();
                verifier.ast_to_symbol.set(exp, Some(r));
                return Ok(Some(r));
            }
        }

        verifier.ast_to_symbol.set(exp, Some(r.clone()));
        Ok(Some(r))
    }

    pub fn to_identifier_name_or_asterisk(&self) -> Option<(String, Location)> {
        if self.attribute || self.qualifier.is_some() {
            None
        } else {
            if let QualifiedIdentifierIdentifier::Id(id) = self.id {
                Some(id.clone())
            } else {
                None
            }
        }
    }

    pub fn to_identifier_name(&self) -> Option<(String, Location)> {
        if self.attribute || self.qualifier.is_some() {
            None
        } else {
            if let QualifiedIdentifierIdentifier::Id(id) = self.id {
                if id.0 == "*" { None } else { Some(id.clone()) }
            } else {
                None
            }
        }
    }
}