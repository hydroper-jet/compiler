use crate::ns::*;
use std::rc::Rc;

pub struct PropertyResolution<'a>(pub &'a mut SymbolHost);

#[derive(Clone)]
pub enum SemanticPropertyKey {
    String(String),
    Number(f64),
    Value(Symbol),
}

impl SemanticPropertyKey {
    pub fn symbol(&self, host: &mut SymbolHost) -> Symbol {
        match self {
            Self::String(s) => host.factory().create_string_constant(s.clone(), &host.string_type()),
            Self::Number(d) => host.factory().create_number_constant(AbstractRangeNumber::Number(d.clone()), &host.number_type()),
            Self::Value(s) => s.clone(),
        }
    }

    pub fn static_type(&self, host: &mut SymbolHost) -> Symbol {
        match self {
            Self::String(s) => host.string_type(),
            Self::Number(d) => host.number_type(),
            Self::Value(s) => s.static_type(host),
        }
    }

    pub fn string_value(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.clone()),
            Self::Value(s) => {
                if s.is_string_constant() {
                    Some(s.string_value())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    pub fn number_value(&self) -> Option<f64> {
        match self {
            Self::Number(d) => Some(*d),
            Self::Value(d) => {
                if d.is_number_constant() {
                    match d.number_value() {
                        AbstractRangeNumber::Number(d) => Some(d),
                        _ => None,
                    }
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}

impl<'a> PropertyResolution<'a> {
    pub fn resolve_property(&mut self, base: &Symbol, qual: Option<Symbol>, key: SemanticPropertyKey) -> Result<Option<Symbol>, PropertyResolutionError> {
        // 1. If base is a value whose type is one of { XML, XMLList }, return XMLReferenceValue(base, qual, key).
        if base.is_value() && [self.0.xml_type(), self.0.xml_list_type()].contains(&base.static_type(self.0)) {
            let k = key.symbol(self.0);
            return Ok(Some(self.0.factory().create_xml_reference_value(base, qual, &k)));
        }

        // 2. If base is a scope, return ResolveScopeProperty(base, qual, key).
        if base.is_scope() {
            return self.resolve_scope_property(base, qual, key);
        }

        let string_key = key.string_value();
        let number_key = key.number_value();

        // 3. If base is a value whose type is * or if key is not a String or Number constant
        //     1. Return DynamicReferenceValue(base, qual, key)
        if (base.is_value() && base.static_type(self.0) == self.0.any_type()) || !(string_key.is_some() || number_key.is_some()) {
            let k = key.symbol(self.0);
            return Ok(Some(self.0.factory().create_dynamic_reference_value(base, qual, &k)));
        }

        // 4. Return undefined if qual is not undefined.
        if qual.is_some() {
            return Ok(None);
        }

        // 5. If base is a class or enum
        if base.is_class_type() || base.is_enum_type() {
            // Key must be a String constant
            let key = string_key;
            if key.is_none() {
                return Ok(None);
            }
            let key = key.unwrap();

            for class in base.descending_class_hierarchy(self.0) {
                // Throw if unresolved
                class.throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

                let r = class.static_properties(self.0).get(&key);
                if let Some(r) = r {
                    // Throw if unresolved
                    r.throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

                    return Ok(Some(self.0.factory().create_static_reference_value(&class, &r)));
                }
            }
            return Ok(None);
        }

        // 6. If base is a value
        if base.is_value() {
            let base_type = base.static_type(self.0);

            // 6.1. Return undefined if the type of base is void or a nullable type.
            if base_type == self.0.void_type() {
                return Err(PropertyResolutionError::VoidBase);
            }
            if base_type.is_nullable_type() {
                return Err(PropertyResolutionError::NullableBase);
            }

            // 6.2. If key is a String constant
            if let Some(key) = string_key {
                if base_type.is_class_type() || base_type.is_enum_type() {
                    for class in base_type.descending_class_hierarchy(self.0) {
                        // Throw if unresolved
                        class.throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

                        let prop = class.prototype(self.0).get(&key);
                        if let Some(prop) = prop {
                            // Throw if unresolved
                            prop.throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

                            return Ok(Some(self.0.factory().create_instance_reference_value(&base, &prop)));
                        }
                    }
                } else if base_type.is_interface_type() {
                    for itrfc in base_type.all_ascending_types(self.0).iter().rev() {
                        // Throw if unresolved
                        itrfc.throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

                        let prop = itrfc.prototype(self.0).get(&key);
                        if let Some(prop) = prop {
                            // Throw if unresolved
                            prop.throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

                            return Ok(Some(self.0.factory().create_instance_reference_value(&base, &prop)));
                        }
                    }
                }
            }

            // 6.3. For each descending type in the type hierarchy of base
            if base_type.is_class_type() || base_type.is_enum_type() {
                let proxy = base_type.find_proxy(ProxyKind::GetProperty, self.0)?;
                if let Some(proxy) = proxy {
                    let key_type = key.static_type(self.0);
                    let proxy_key_type = proxy.signature(self.0).parameters().get(0).unwrap().static_type.clone();
                    if key_type.is_equals_or_subtype_of(&proxy_key_type, self.0) {
                        return Ok(Some(self.0.factory().create_proxy_reference_value(&base, &proxy)));
                    }
                }
            }

            // 6.4. If key is a Number constant value and base is of a tuple type
            if number_key.is_some() && base_type.is_tuple_type() {
                let index: usize = unsafe { number_key.unwrap().to_int_unchecked() };
                if index >= base_type.element_types().length() {
                    return Ok(None);
                }
                return Ok(Some(self.0.factory().create_tuple_reference_value(&base, index)));
            }

            return Ok(None);
        }

        // 7. If base is a package
        if base.is_package() {
            // Key must be a String constant
            let key = string_key;
            if key.is_none() {
                return Ok(None);
            }
            let key = key.unwrap();

            let r = base.properties(self.0).get(&key);
            if let Some(r) = r {
                return Ok(Some(r.resolve_alias().wrap_property_reference(self.0)));
            }
            for p in base.redirect_packages().iter() {
                let r = self.resolve_property(&p, qual.clone(), SemanticPropertyKey::String(key.clone()))?;
                if r.is_some() {
                    return Ok(r);
                }
            }
            return Ok(None);
        }

        // 8. If base is a package set
        if base.is_package_set() {
            // Key must be a String constant
            let key = string_key;
            if key.is_none() {
                return Ok(None);
            }
            let key = key.unwrap();

            for p in base.packages().iter() {
                let r = self.resolve_property(&p, qual.clone(), SemanticPropertyKey::String(key.clone()))?;
                if r.is_some() {
                    return Ok(r);
                }
            }
            return Ok(None);
        }

        // 9. If base is the import.meta symbol
        if base.is_import_meta() {
            // Key must be a String constant
            let key = string_key;
            if key.is_none() {
                return Ok(None);
            }
            let key: &str = key.unwrap().as_ref();
            match key {
                "env" => {
                    return Ok(Some(self.0.import_meta_env()));
                },
                "output" => {
                    return Ok(Some(self.0.factory().create_import_meta_output_value()));
                },
                _ => {
                    return Ok(None);
                },
            }
        }

        // 10. If base is the import.meta.env symbol
        if base.is_import_meta_env() {
            zxc_zxc_zxc;
        }

        // 11.
        return Ok(None);
    }
}