use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ObjectInitializer {
    pub location: Location,
    pub fields: Vec<Rc<InitializerField>>,
}

impl ObjectInitializer {
    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier, context: &ExpressionVerifyContext) -> Result<Option<Symbol>, DeferVerificationError> {
        let context_type = context.context_type.clone().unwrap_or(verifier.host.map_type_of_any_any());
        context_type.throw_if_unresolved()?;
        let context_type_non_null = context_type.non_null_type();

        let object_type = verifier.host.object_type();
        object_type.throw_if_unresolved()?;

        let map_type_of_any_any = verifier.host.map_type_of_any_any();
        map_type_of_any_any.throw_if_unresolved()?;

        verifier.host.string_type().throw_if_unresolved()?;
        verifier.host.number_type().throw_if_unresolved()?;
        verifier.host.boolean_type().throw_if_unresolved()?;

        if [verifier.host.any_type(), object_type, map_type_of_any_any].contains(&context_type_non_null) {
            self.verify_any_or_object(verifier, &context_type)
        } else {
            let k_v_types = context_type_non_null.map_key_value_types(&verifier.host)?;
            if let Some((k_t, v_t)) = k_v_types {
                self.verify_map(verifier, &context_type, &k_t, &v_t)
            } else if context_type_non_null.is_enum_type() && context_type_non_null.is_set_enumeration() {
                self.verify_set_enum(verifier, &context_type)
            } else if context_type_non_null.is_class_type() && context_type_non_null.allow_literal() {
                self.verify_literal_class(verifier, &context_type)
            } else {
                self.verify_failure(verifier, &context_type)
            }
        }
    }

    fn verify_any_or_object(&self, verifier: &mut VerifierVerifier, context_type: &Symbol) -> Result<Option<Symbol>, DeferVerificationError> {
        for field in &self.fields {
            match field.as_ref() {
                InitializerField::Rest((exp, _)) => {
                    let map_type_of_any_any = verifier.host.map_type_of_any_any();
                    verifier.limit_expression_type(exp, &map_type_of_any_any)?;
                },
                InitializerField::Field { name, value, .. } => {
                    if let Some(name) = field.shorthand() {
                        Self::resolve_shorthand(verifier, &name)?;
                    } else {
                        if let FieldName::Brackets(exp) = &name.0 {
                            verifier.verify_expression(exp, &default())?;
                        }
                        verifier.verify_expression(value.as_ref().unwrap(), &default())?;
                    }
                },
            }
        }
        Ok(Some(verifier.host.factory().create_value(context_type)))
    }

    fn verify_map(&self, verifier: &mut VerifierVerifier, context_type: &Symbol, k_t: &Symbol, v_t: &Symbol) -> Result<Option<Symbol>, DeferVerificationError> {
        let map_type = context_type.non_null_type();
        for field in &self.fields {
            match field.as_ref() {
                InitializerField::Rest((exp, _)) => {
                    verifier.limit_expression_type(exp, &map_type)?;
                },
                InitializerField::Field { name, value, .. } => {
                    if let Some(name) = field.shorthand() {
                        let short_ref = Self::resolve_shorthand(verifier, &name)?;
                        if let Some(short_ref_1) = short_ref {
                            if TypeConversions(&verifier.host).implicit_conversion(&short_ref_1, v_t, false).is_none() {
                                verifier.add_verify_error(&name.1, DiagnosticKind::IncompatibleTypes, diagnostic_arguments![Symbol(v_t.clone()), Symbol(short_ref_1.static_type(&verifier.host))]);
                            }
                        }
                        if ![verifier.host.any_type(), verifier.host.object_type(), verifier.host.string_type()].contains(&k_t) {
                            verifier.add_verify_error(&name.1, DiagnosticKind::IncompatibleFieldKey, diagnostic_arguments![]);
                        }
                    } else {
                        match &name.0 {
                            FieldName::Brackets(exp) => {
                                verifier.limit_expression_type(exp, k_t)?;
                            },
                            FieldName::Identifier(_) | FieldName::StringLiteral(_) => {
                                if ![verifier.host.any_type(), verifier.host.object_type(), verifier.host.string_type()].contains(&k_t) {
                                    verifier.add_verify_error(&name.1, DiagnosticKind::IncompatibleFieldKey, diagnostic_arguments![]);
                                }
                            },
                            FieldName::NumericLiteral(_) => {
                                if ![verifier.host.any_type(), verifier.host.object_type(), verifier.host.number_type()].contains(&k_t) {
                                    verifier.add_verify_error(&name.1, DiagnosticKind::IncompatibleFieldKey, diagnostic_arguments![]);
                                }
                            },
                        }
                        verifier.limit_expression_type(value.as_ref().unwrap(), v_t)?;
                    }
                },
            }
        }
        Ok(Some(verifier.host.factory().create_value(context_type)))
    }

    fn verify_set_enum(&self, verifier: &mut VerifierVerifier, context_type: &Symbol) -> Result<Option<Symbol>, DeferVerificationError> {
        let enum_type = context_type.non_null_type();
        let repr_type = enum_type.enumeration_representation_type().unwrap();
        repr_type.throw_if_unresolved()?;
        let mut c = AbstractRangeNumber::zero(&repr_type, &verifier.host);
        let mut is_const = false;

        let boolean_type  = verifier.host.boolean_type();
        let string_type  = verifier.host.string_type();

        for field in &self.fields {
            match field.as_ref() {
                InitializerField::Rest((exp, _)) => {
                    let c1 = verifier.limit_expression_type(exp, &enum_type)?;
                    if let Some(c1) = c1 {
                        if c1.is_enum_constant() && is_const {
                            c = c | c1.number_value();
                        } else {
                            is_const = false;
                        }
                    }
                },
                InitializerField::Field { .. } => {
                    if let Some(name) = field.shorthand() {
                        let mut short_ref = Self::resolve_shorthand(verifier, &name)?;
                        if let Some(short_ref_1) = short_ref.as_ref() {
                            if TypeConversions(&verifier.host).implicit_conversion(short_ref_1, &boolean_type, false).is_none() {
                                verifier.add_verify_error(&name.1, DiagnosticKind::IncompatibleTypes, diagnostic_arguments![Symbol(boolean_type.clone()), Symbol(short_ref_1.static_type(&verifier.host))]);
                                short_ref = None;
                            }
                        }
                        let member = enum_type.enumeration_members().get(&name.0);
                        if let Some(member) = member {
                            if let Some(short_ref) = short_ref {
                                if short_ref.is_boolean_constant() && is_const {
                                    c = c.apply_bits(&member, short_ref.boolean_value());
                                } else {
                                    is_const = false;
                                }
                            }
                        } else {
                            verifier.add_verify_error(&name.1, DiagnosticKind::EnumerationHasNoMember, diagnostic_arguments![Symbol(enum_type.clone()), String(name.0.clone())]);
                        }
                    } else {
                        let (is_const_1, c1) = Self::verify_set_enum_non_shorthand_notation(field, verifier, &enum_type, &boolean_type, &string_type, is_const, c.clone())?;
                        is_const = is_const_1;
                        c = c1;
                    }
                },
            }
        }

        if is_const {
            return Ok(Some(verifier.host.factory().create_enum_constant(c, &context_type)));
        }

        Ok(Some(verifier.host.factory().create_value(context_type)))
    }

    fn verify_set_enum_non_shorthand_notation(field: &Rc<InitializerField>, verifier: &mut VerifierVerifier, enum_type: &Symbol, boolean_type: &Symbol, string_type: &Symbol, mut is_const: bool, mut c: AbstractRangeNumber) -> Result<(bool, AbstractRangeNumber), DeferVerificationError> {
        let InitializerField::Field { name, value, .. } = field.as_ref() else {
            panic!();
        };
        let value_exp = value.as_ref().unwrap();
        let c1 = verifier.limit_expression_type(value_exp, &boolean_type)?;
        match &name.0 {
            FieldName::Brackets(exp) => {
                verifier.limit_expression_type(exp, &string_type)?;
                is_const = false;
            },
            FieldName::Identifier(name_1) => {
                let member = enum_type.enumeration_members().get(name_1);
                if let Some(member) = member {
                    if let Some(c1) = c1 {
                        if c1.is_boolean_constant() && is_const {
                            c = c.apply_bits(&member, c1.boolean_value());
                        } else {
                            is_const = false;
                        }
                    }
                } else {
                    verifier.add_verify_error(&name.1, DiagnosticKind::EnumerationHasNoMember, diagnostic_arguments![Symbol(enum_type.clone()), String(name_1.clone())]);
                }
            },
            FieldName::StringLiteral(sl) => {
                let name_1 = verifier.verify_expression(sl, &default())?.unwrap().string_value();
                let member = enum_type.enumeration_members().get(&name_1);
                if let Some(member) = member {
                    if let Some(c1) = c1 {
                        if c1.is_boolean_constant() && is_const {
                            c = c.apply_bits(&member, c1.boolean_value());
                        } else {
                            is_const = false;
                        }
                    }
                } else {
                    verifier.add_verify_error(&name.1, DiagnosticKind::EnumerationHasNoMember, diagnostic_arguments![Symbol(enum_type.clone()), String(name_1)]);
                }
            },
            FieldName::NumericLiteral(_) => {
                verifier.add_verify_error(&name.1, DiagnosticKind::IncompatibleFieldKey, diagnostic_arguments![]);
            },
        }
        Ok((is_const, c))
    }

    fn verify_literal_class(&self, verifier: &mut VerifierVerifier, context_type: &Symbol) -> Result<Option<Symbol>, DeferVerificationError> {
        let c = context_type.non_null_type();
        
        let mut missing = HashSet::<Symbol>::new();
        for (_, prop) in c.prototype(&verifier.host).borrow().iter() {
            if prop.is_variable_property() && !prop.is_optional_variable(&verifier.host)? {
                missing.insert(prop.clone());
            }
        }

        for field in &self.fields {
            match field.as_ref() {
                InitializerField::Rest((exp, _)) => {
                    verifier.limit_expression_type(exp, &c)?;
                    missing.clear();
                },
                InitializerField::Field { .. } => {
                    if let Some(name) = field.shorthand() {
                        let variable = Self::resolve_instance_variable(verifier, &c, &name)?;
                        if let Some(variable) = variable.clone() {
                            missing.remove(&variable);
                        }
                        let mut short_ref = Self::resolve_shorthand(verifier, &name)?;
                        if let Some(short_ref_1) = short_ref.as_ref() {
                            if let Some(variable) = variable {
                                let variable_data_type = variable.static_type(&verifier.host);
                                variable_data_type.throw_if_unresolved()?;
                                if TypeConversions(&verifier.host).implicit_conversion(short_ref_1, &variable_data_type, false).is_none() {
                                    verifier.add_verify_error(&name.1, DiagnosticKind::IncompatibleTypes, diagnostic_arguments![Symbol(variable_data_type), Symbol(short_ref_1.static_type(&verifier.host))]);
                                    #[allow(unused_assignments)] {
                                        short_ref = None;
                                    }
                                }
                            }
                        }
                    } else {
                        Self::verify_literal_class_non_shorthand_notation(field, verifier, &c, &mut missing)?;
                    }
                },
            }
        }

        for m in &missing {
            verifier.add_verify_error(&self.location, DiagnosticKind::MissingPropertyInLiteral, diagnostic_arguments![String(m.name())]);
        }

        Ok(Some(verifier.host.factory().create_value(context_type)))
    }

    fn verify_literal_class_non_shorthand_notation(field: &Rc<InitializerField>, verifier: &mut VerifierVerifier, c: &Symbol, missing: &mut HashSet<Symbol>) -> Result<(), DeferVerificationError> {
        let InitializerField::Field { name, value, .. } = field.as_ref() else {
            panic!();
        };
        let value_exp = value.as_ref().unwrap();
        match &name.0 {
            FieldName::Brackets(exp) => {
                verifier.limit_expression_type(exp, &verifier.host.string_type())?;
                verifier.verify_expression(value_exp, &default());
                missing.clear();
            },
            FieldName::Identifier(name_1) => {
                let variable = Self::resolve_instance_variable(verifier, &c, &(name_1.clone(), name.1.clone()))?;
                if let Some(variable) = variable.clone() {
                    missing.remove(&variable);
                }
                if let Some(variable) = variable {
                    let variable_data_type = variable.static_type(&verifier.host);
                    if variable_data_type.is_unresolved() {
                        verifier.verify_expression(value_exp, &default())?;
                    } else {
                        verifier.limit_expression_type(value_exp, &variable_data_type);
                    }
                } else {
                    verifier.verify_expression(value_exp, &default())?;
                }
            },
            FieldName::StringLiteral(sl) => {
                let name_1 = verifier.verify_expression(sl, &default())?.unwrap().string_value();
                let variable = Self::resolve_instance_variable(verifier, &c, &(name_1, name.1.clone()))?;
                if let Some(variable) = variable.clone() {
                    missing.remove(&variable);
                }
                if let Some(variable) = variable {
                    let variable_data_type = variable.static_type(&verifier.host);
                    if variable_data_type.is_unresolved() {
                        verifier.verify_expression(value_exp, &default())?;
                    } else {
                        verifier.limit_expression_type(value_exp, &variable_data_type);
                    }
                } else {
                    verifier.verify_expression(value_exp, &default())?;
                }
            },
            FieldName::NumericLiteral(_) => {
                verifier.verify_expression(value_exp, &default());
                verifier.add_verify_error(&name.1, DiagnosticKind::IncompatibleFieldKey, diagnostic_arguments![]);
            },
        }
        Ok(())
    }

    fn verify_failure(&self, verifier: &mut VerifierVerifier, context_type: &Symbol) -> Result<Option<Symbol>, DeferVerificationError> {
        verifier.add_verify_error(&self.location, DiagnosticKind::InitializerUnsupportedType, diagnostic_arguments![Symbol(context_type.clone())]);
        for field in &self.fields {
            match field.as_ref() {
                InitializerField::Rest((exp, _)) => {
                    verifier.verify_expression(exp, &default())?;
                },
                InitializerField::Field { name, value, .. } => {
                    if let Some(name) = field.shorthand() {
                        Self::resolve_shorthand(verifier, &name)?;
                    } else {
                        if let FieldName::Brackets(exp) = &name.0 {
                            verifier.verify_expression(exp, &default())?;
                        }
                        verifier.verify_expression(value.as_ref().unwrap(), &default())?;
                    }
                },
            }
        }
        Ok(None)
    }

    fn resolve_shorthand(verifier: &mut VerifierVerifier, name: &(String, Location)) -> Result<Option<Symbol>, DeferVerificationError> {
        let name_str = &name.0;
        let r = verifier.scope.resolve_property(None, SemanticPropertyKey::String(name_str.clone()), &verifier.host);
        if r.is_err() {
            match r.unwrap_err() {
                PropertyResolutionError::AmbiguousReference { name: amb_name } => {
                    verifier.add_verify_error(&name.1, DiagnosticKind::AmbiguousReference, diagnostic_arguments![String(amb_name.clone())]);
                    return Ok(None);
                },
                PropertyResolutionError::DeferVerification => {
                    return Err(DeferVerificationError);
                },
                PropertyResolutionError::VoidBase => {
                    verifier.add_verify_error(&name.1, DiagnosticKind::AccessingPropertyOfVoidBase, diagnostic_arguments![]);
                    return Ok(None);
                },
                PropertyResolutionError::NullableBase { nullable_type } => {
                    verifier.add_verify_error(&name.1, DiagnosticKind::AccessingPropertyOfNullableBase, diagnostic_arguments![Symbol(nullable_type)]);
                    return Ok(None);
                },
            }
        }
        let r = r.unwrap();
        if r.is_none() {
            verifier.add_verify_error(&name.1, DiagnosticKind::UndefinedProperty, diagnostic_arguments![String(name_str.clone())]);
            return Ok(None);
        }
        let r = r.unwrap();

        if !r.property_is_visible(&verifier.scope, &verifier.host) {
            verifier.add_verify_error(&name.1, DiagnosticKind::InaccessibleProperty, diagnostic_arguments![String(name_str.clone())]);
        }

        if r.is_reference_value() && (r.is_static_reference_value() || r.is_instance_reference_value() || r.is_scope_reference_value() || r.is_package_reference_value()) {
            let p = r.property();

            // Require type arguments
            if (p.is_origin_function() || p.is_origin_class_type() || p.is_origin_interface_type()) && p.type_parameters().is_some() {
                verifier.add_verify_error(&name.1, DiagnosticKind::TypeParameterizedPropertyMustBeArgumented, diagnostic_arguments![]);
            }
        }

        Ok(Some(r))
    }

    fn resolve_instance_variable(verifier: &mut VerifierVerifier, c: &Symbol, name: &(String, Location)) -> Result<Option<Symbol>, DeferVerificationError> {
        let variable = c.prototype(&verifier.host).get(&name.0).and_then(|v| if v.is_variable_property() { Some(v) } else { None });
        if variable.is_none() {
            verifier.add_verify_error(&name.1, DiagnosticKind::UndefinedProperty, diagnostic_arguments![String(name.0.clone())]);
            return Ok(None);
        }
        let variable = variable.unwrap();
        if !variable.property_is_visible(&verifier.scope, &verifier.host) {
            verifier.add_verify_error(&name.1, DiagnosticKind::InaccessibleProperty, diagnostic_arguments![String(name.0.clone())]);
        }
        Ok(Some(variable))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum InitializerField {
    Field {
        name: (FieldName, Location),
        /// Non-null operator used for destructuring.
        non_null: bool,
        value: Option<Rc<Expression>>,
    },
    Rest((Rc<Expression>, Location)),
}

impl InitializerField {
    pub fn location(&self) -> Location {
        match self {
            Self::Field { ref name, ref value, .. } => {
                value.clone().map_or(name.1.clone(), |v| name.1.combine_with(v.location()))
            },
            Self::Rest((_, ref l)) => l.clone(),
        }
    }

    pub fn shorthand(&self) -> Option<(String, Location)> {
        if let Self::Field { name, .. } = self {
            if let FieldName::Identifier(name_str) = &name.0 {
                Some((name_str.clone(), name.1.clone()))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FieldName {
    Identifier(String),
    Brackets(Rc<Expression>),
    StringLiteral(Rc<Expression>),
    NumericLiteral(Rc<Expression>),
}

impl FieldName {
    pub(crate) fn id(&self) -> Option<String> {
        let Self::Identifier(id) = &self else {
            return None;
        };
        Some(id.clone())
    }

    pub(crate) fn id_equals(&self, name: &str) -> bool {
        self.id().map(|name1| name == name1).unwrap_or(false)
    }
}