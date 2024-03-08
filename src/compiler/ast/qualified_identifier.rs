use crate::ns::*;
use serde::{Serialize, Deserialize};

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
    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier) -> Result<Option<(Option<Symbol>, SemanticPropertyKey, PropertyDisambiguation)>, DeferVerificationError> {
        let QualifiedIdentifier {
            qualifier,
            id,
            ..
        } = self;

        let mut failed = false;
        let mut disamb = PropertyDisambiguation::Default;

        let mut result_qual: Option<Symbol> = None;
        if let Some(qualifier) = qualifier {
            let q = qualifier.to_identifier_name();
            if let Some(q) = q {
                if q.0 == "fixed" && q.1.character_count() == "fixed".len() {
                    disamb = PropertyDisambiguation::Fixed;
                } else if q.0 == "dynamic" && q.1.character_count() == "dynamic".len() {
                    disamb = PropertyDisambiguation::Dynamic;
                }
            }
            if disamb == PropertyDisambiguation::Default {
                result_qual = verifier.limit_expression_type(qualifier, &verifier.host.namespace_type())?;
                if result_qual.is_none() {
                    failed = true;
                }
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
            Ok(Some((result_qual, result_key.unwrap(), disamb)))
        }
    }

    pub(crate) fn verify_as_exp(&self, verifier: &mut VerifierVerifier, context: &ExpressionVerifyContext) -> Result<Option<Symbol>, DeferVerificationError> {
        let qn = self.verify(verifier)?;
        if qn.is_none() {
            return Ok(None);
        }
        let (qual, key, disamb) = qn.unwrap();
        let r = verifier.scope.resolve_property_with_disambiguation(qual, key.clone(), &verifier.host, disamb);
        if r.is_err() {
            match r.unwrap_err() {
                PropertyResolutionError::AmbiguousReference { name } => {
                    verifier.add_verify_error(&self.location, DiagnosticKind::AmbiguousReference, diagnostic_arguments![String(name.clone())]);
                    return Ok(None);
                },
                PropertyResolutionError::DeferVerification => {
                    return Err(DeferVerificationError);
                },
                PropertyResolutionError::VoidBase => {
                    verifier.add_verify_error(&self.location, DiagnosticKind::AccessingPropertyOfVoidBase, diagnostic_arguments![]);
                    return Ok(None);
                },
                PropertyResolutionError::NullableBase { nullable_type } => {
                    verifier.add_verify_error(&self.location, DiagnosticKind::AccessingPropertyOfNullableBase, diagnostic_arguments![Symbol(nullable_type)]);
                    return Ok(None);
                },
            }
        }
        let r = r.unwrap();
        if r.is_none() {
            verifier.add_verify_error(&self.location, DiagnosticKind::UndefinedProperty, diagnostic_arguments![String(key.string_value().unwrap_or(key.number_value().unwrap().to_string()))]);
            return Ok(None);
        }
        let r = r.unwrap();

        if !r.property_is_visible(&verifier.scope, &verifier.host) {
            verifier.add_verify_error(&self.location, DiagnosticKind::InaccessibleProperty, diagnostic_arguments![String(key.string_value().unwrap())]);
        }

        if r.is_reference_value() && (r.is_static_reference_value() || r.is_instance_reference_value() || r.is_scope_reference_value() || r.is_package_reference_value()) {
            let p = r.property();

            // Require type arguments
            if (p.is_origin_function() || p.is_origin_class_type() || p.is_origin_interface_type()) && p.type_parameters().is_some() && !context.followed_by_type_arguments {
                verifier.add_verify_error(&self.location, DiagnosticKind::TypeParameterizedPropertyMustBeArgumented, diagnostic_arguments![]);
            }

            // Compile-time constant
            if p.is_origin_variable_property() && p.read_only(&verifier.host) && p.constant_initializer().is_some() {
                let r = p.constant_initializer().unwrap();
                return Ok(Some(r));
            }
        }

        Ok(Some(r))
    }

    pub fn to_identifier_name_or_asterisk(&self) -> Option<(String, Location)> {
        if self.attribute || self.qualifier.is_some() {
            None
        } else {
            if let QualifiedIdentifierIdentifier::Id(id) = &self.id {
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
            if let QualifiedIdentifierIdentifier::Id(id) = &self.id {
                if id.0 == "*" { None } else { Some(id.clone()) }
            } else {
                None
            }
        }
    }
}