use crate::ns::*;

pub struct MethodOverriding<'a>(pub &'a SymbolHost);

impl<'a> MethodOverriding<'a> {
    pub fn abstract_methods_not_overriden(&mut self, class: &Symbol) -> Result<Vec<Symbol>, DeferVerificationError> {
        let base_class = class.extends_class(self.0);
        if base_class.is_none() {
            return Ok(vec![]);
        }
        let base_class = base_class.unwrap();
        if base_class.is_unresolved() {
            return Err(DeferVerificationError);
        }
        let mut r: Vec<Symbol> = vec![];
        for (name, prop) in base_class.prototype(self.0).borrow().iter() {
            // Regular method
            if prop.is_function() {
                if prop.is_abstract() {
                    let prop2 = class.prototype(self.0).get(name);
                    if prop2.is_none() || !prop2.unwrap().is_function() {
                        r.push(prop.clone());
                    }
                }
            }
            // Accessor
            else if prop.is_virtual_property() {
                if let Some(getter) = prop.getter(self.0) {
                    if prop.not_overriden_abstract_getter(&getter, class, self.0) {
                        r.push(prop.clone());
                    }
                }
                if let Some(setter) = prop.setter(self.0) {
                    if prop.not_overriden_abstract_setter(&setter, class, self.0) {
                        r.push(prop.clone());
                    }
                }
            }
        }
        Ok(r)
    }

    pub fn override_method(&mut self, method: &Symbol) -> Result<(), MethodOverridingError> {
        let name = method.name();
        let class = method.parent_definition().unwrap();
        assert!(class.is_class_type() || class.is_enum_type());

        let base_type = class.extends_class(self.0);
        if base_type.is_none() {
            return Err(MethodOverridingError::MustOverrideAMethod);
        }
        let base_type = base_type.unwrap();
        let base_method = self.lookup_method(&name, &base_type)?;
        if base_method.is_none() {
            return Err(MethodOverridingError::MustOverrideAMethod);
        }
        let mut base_method = base_method.unwrap();
        
        let virtual_property = method.of_virtual_property(self.0);
        if let Some(virtual_property) = virtual_property {
            let is_getter = Some(method.clone()) == virtual_property.getter(self.0);
            if is_getter {
                // Overriding a getter
                if !(base_method.is_virtual_property() && base_method.getter(self.0).is_some()) {
                    return Err(MethodOverridingError::MustOverrideAMethod);
                }
                base_method = base_method.getter(self.0).unwrap();
            } else {
                // Overriding a setter
                if !(base_method.is_virtual_property() && base_method.setter(self.0).is_some()) {
                    return Err(MethodOverridingError::MustOverrideAMethod);
                }
                base_method = base_method.setter(self.0).unwrap();
            }
        // Overriding a regular method
        } else if !base_method.is_function() {
            return Err(MethodOverridingError::MustOverrideAMethod);
        }

        if base_method.type_parameters().is_some() {
            return Err(MethodOverridingError::CannotOverrideTypeParameterizedMethod);
        }

        if method.type_parameters().is_some() {
            return Err(MethodOverridingError::CannotIntroduceTypeParameters);
        }

        // Retrieve base type method's signature. Throw if unresolved.
        let base_signature = base_method.signature(self.0);
        base_signature.throw_if_unresolved().map_err(|_| MethodOverridingError::DeferVerification)?;

        // Retrieve subtype method's signature. Throw if unresolved.
        let subtype_signature = method.signature(self.0);
        subtype_signature.throw_if_unresolved().map_err(|_| MethodOverridingError::DeferVerification)?;

        if !self.overriding_signature_is_compatible(&base_signature, &subtype_signature) {
            return Err(MethodOverridingError::IncompatibleSignature {
                expected_signature: base_signature,
                actual_signature: subtype_signature,
            });
        }

        if base_method.is_final() {
            return Err(MethodOverridingError::OverridingFinalMethod);
        }

        base_method.overriden_by(self.0).push(method.clone());
        method.set_overrides_method(Some(base_method));
        Ok(())
    }

    fn lookup_method(&mut self, name: &String, base_type: &Symbol) -> Result<Option<Symbol>, MethodOverridingError> {
        for class in base_type.descending_class_hierarchy(self.0).collect::<Vec<_>>() {
            // Throw if unresolved
            class.throw_if_unresolved().map_err(|_| MethodOverridingError::DeferVerification)?;

            let prop = class.prototype(self.0).get(name);
            if let Some(prop) = prop {
                // Throw if unresolved
                prop.property_static_type(self.0).throw_if_unresolved().map_err(|_| MethodOverridingError::DeferVerification)?;

                if prop.is_virtual_property() {
                    if let Some(getter) = prop.getter(self.0) {
                        // Throw if unresolved
                        getter.signature(self.0).throw_if_unresolved().map_err(|_| MethodOverridingError::DeferVerification)?;
                    }

                    if let Some(setter) = prop.setter(self.0) {
                        // Throw if unresolved
                        setter.signature(self.0).throw_if_unresolved().map_err(|_| MethodOverridingError::DeferVerification)?;
                    }
                }

                return Ok(Some(prop));
            }
        }
        Ok(None)
    }

    fn overriding_signature_is_compatible(&mut self, base_signature: &Symbol, subtype_signature: &Symbol) -> bool {
        if base_signature == subtype_signature {
            return true;
        }

        let base_parameters = base_signature.parameters();
        let mut base_parameters = base_parameters.iter();
        let subtype_parameters = subtype_signature.parameters();
        let mut subtype_parameters = subtype_parameters.iter();

        while let Some(base_param) = base_parameters.next() {
            let subtype_param = subtype_parameters.next();
            if subtype_param.is_none() {
                return false;
            }
            let subtype_param = subtype_param.unwrap();

            if !(base_param.kind == subtype_param.kind && base_param.static_type == subtype_param.static_type) {
                return false;
            }
        }

        let cont_param = subtype_parameters.next();
        if cont_param.is_some() && cont_param.unwrap().kind == ParameterKind::Required {
            return false;
        }

        subtype_signature.result_type().is_equals_or_subtype_of(&base_signature.result_type(), self.0)
    }
}