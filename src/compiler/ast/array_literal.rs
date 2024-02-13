use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ArrayLiteral {
    pub location: Location,
    pub elements: Vec<Element>,
}

impl ArrayLiteral {
    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier, context: &ExpressionVerifyContext) -> Result<Option<Symbol>, DeferVerificationError> {
        let context_type = context.context_type.clone().unwrap_or(verifier.host.array_type_of_any());
        context_type.throw_if_unresolved()?;
        let context_type_non_null = context_type.non_null_type();

        let object_type = verifier.host.object_type();
        object_type.throw_if_unresolved()?;

        let array_type_of_any = verifier.host.array_type_of_any();
        array_type_of_any.throw_if_unresolved()?;

        if [verifier.host.any_type(), object_type, array_type_of_any].contains(&context_type_non_null) {
            for elem in &self.elements {
                match elem {
                    Element::Rest((exp, _)) => {
                        ArrayLiteral::verify_rest(exp, verifier, &verifier.host.any_type())?;
                    },
                    Element::Expression(exp) => {
                        verifier.verify_expression(exp, &default())?;
                    },
                    _ => {},
                }
            }
        } else if context_type_non_null.is_tuple_type() {
            let mut elision_found = false;
            let mut i: usize = 0;
            let tuple_type = context_type_non_null.clone();
            if self.elements.len() > tuple_type.element_types().length() {
                verifier.add_syntax_error(&self.location, DiagnosticKind::ArrayLiteralExceedingTupleElements, diagnostic_arguments![Symbol(tuple_type.clone())]);
            }
            for elem in &self.elements {
                match elem {
                    Element::Elision => {
                        elision_found = true;
                    },
                    Element::Rest((exp, loc)) => {
                        verifier.verify_expression(exp, &default())?;
                        verifier.add_syntax_error(loc, DiagnosticKind::ArrayLiteralMustNotContainRest, diagnostic_arguments![]);
                    },
                    Element::Expression(exp) => {
                        let element_type = tuple_type.element_types().get(i);
                        if let Some(element_type) = element_type {
                            verifier.limit_expression_type(exp, &element_type)?;
                        } else {
                            verifier.verify_expression(exp, &default())?;
                        }
                    },
                }
                i += 1;
            }
            if elision_found {
                verifier.add_syntax_error(&self.location, DiagnosticKind::ArrayLiteralMustNotContainElision, diagnostic_arguments![]);
            }
        } else if context_type_non_null.is_enum_type() && context_type_non_null.is_set_enumeration() {
            let mut elision_found = false;
            let enum_type = context_type_non_null.clone();
            let repr_type = enum_type.enumeration_representation_type().unwrap();
            repr_type.throw_if_unresolved()?;
            let mut c = AbstractRangeNumber::zero(&repr_type, &verifier.host);
            let mut is_const = false;
            for elem in &self.elements {
                match elem {
                    Element::Elision => {
                        elision_found = true;
                    },
                    Element::Rest((exp, loc)) => {
                        verifier.verify_expression(exp, &default())?;
                        verifier.add_syntax_error(loc, DiagnosticKind::ArrayLiteralMustNotContainRest, diagnostic_arguments![]);
                    },
                    Element::Expression(exp) => {
                        let c1 = verifier.limit_expression_type(exp, &enum_type)?;
                        if let Some(c1) = c1 {
                            if c1.is_enum_constant() && is_const {
                                c = c | c1.number_value();
                            } else {
                                is_const = false;
                            }
                        }
                    },
                }
            }
            if elision_found {
                verifier.add_syntax_error(&self.location, DiagnosticKind::ArrayLiteralMustNotContainElision, diagnostic_arguments![]);
            }
            if is_const {
                return Ok(Some(verifier.host.factory().create_enum_constant(c, &context_type)));
            }
        } else {
            let element_type = context_type_non_null.array_element_type(&verifier.host)?;
            if let Some(element_type) = element_type {
                let mut elision_found = false;
                for elem in &self.elements {
                    match elem {
                        Element::Elision => {
                            elision_found = true;
                        },
                        Element::Rest((exp, _)) => {
                            ArrayLiteral::verify_rest(exp, verifier, &element_type);
                        },
                        Element::Expression(exp) => {
                            verifier.limit_expression_type(exp, &element_type)?;
                        },
                    }
                }
                if elision_found {
                    verifier.add_syntax_error(&self.location, DiagnosticKind::ArrayLiteralMustNotContainElision, diagnostic_arguments![]);
                }
            } else {
                verifier.add_syntax_error(&self.location, DiagnosticKind::InitializerUnsupportedType, diagnostic_arguments![Symbol(context_type.clone())]);
                for elem in &self.elements {
                    match elem {
                        Element::Rest((exp, _)) => {
                            ArrayLiteral::verify_rest(exp, verifier, &verifier.host.any_type())?;
                        },
                        Element::Expression(exp) => {
                            verifier.verify_expression(exp, &default())?;
                        },
                        _ => {},
                    }
                }
            }
        }

        Ok(Some(verifier.host.factory().create_value(&context_type)))
    }

    fn verify_rest(exp: &Rc<Expression>, verifier: &mut VerifierVerifier, item_type: &Symbol) -> Result<(), DeferVerificationError> {
        let val = verifier.verify_expression(exp, &ExpressionVerifyContext {
            context_type: Some(item_type.clone()),
            ..default()
        })?;
        if val.is_none() {
            return Ok(());
        }
        let val_type = val.clone().unwrap().static_type(&verifier.host);
        let proxy = if val_type.is_class_type() || val_type.is_enum_type() {
            val_type.find_proxy(ProxyKind::Values, &verifier.host)?
        } else {
            None
        };
        let iterator_type = verifier.host.factory().create_type_after_explicit_type_substitution(&verifier.host.iterator_type(), &shared_array![item_type.clone()]);
        let mut can_be_used_in_rest = false;
        if let Some(proxy) = proxy {
            let proxy_signature = proxy.signature(&verifier.host);
            proxy_signature.throw_if_unresolved()?;
            if proxy_signature.result_type().is_equals_or_subtype_of(&iterator_type, &verifier.host) {
                can_be_used_in_rest = true;
            }
        } else {
            if val_type.is_equals_or_subtype_of(&iterator_type, &verifier.host) {
                can_be_used_in_rest = true;
            }
        }

        if !can_be_used_in_rest {
            verifier.add_verify_error(&exp.location(), DiagnosticKind::CannotUseTypeInRest, diagnostic_arguments![Symbol(val_type.clone())]);
        }

        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Element {
    Elision,
    Expression(Rc<Expression>),
    Rest((Rc<Expression>, Location)),
}