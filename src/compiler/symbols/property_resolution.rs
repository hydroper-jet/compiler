use crate::ns::*;

pub struct PropertyResolution<'a>(pub &'a SymbolHost);

#[derive(Clone)]
pub enum SemanticPropertyKey {
    String(String),
    Number(f64),
    Value(Symbol),
}

impl SemanticPropertyKey {
    pub fn symbol(&self, host: &SymbolHost) -> Symbol {
        match self {
            Self::String(s) => {
                let string_type = host.string_type();
                host.factory().create_string_constant(s.clone(), &string_type)
            },
            Self::Number(d) => {
                let number_type = host.number_type();
                host.factory().create_number_constant(AbstractRangeNumber::Number(d.clone()), &number_type)
            },
            Self::Value(s) => s.clone(),
        }
    }

    pub fn static_type(&self, host: &SymbolHost) -> Symbol {
        match self {
            Self::String(_) => host.string_type(),
            Self::Number(_) => host.number_type(),
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

#[derive(Copy, Clone, PartialEq)]
pub enum PropertyDisambiguation {
    Default,
    Fixed,
    Dynamic,
}

impl<'a> PropertyResolution<'a> {
    pub fn resolve_property(&mut self, base: &Symbol, qual: Option<Symbol>, key: SemanticPropertyKey) -> Result<Option<Symbol>, PropertyResolutionError> {
        self.resolve_property_with_disambiguation(base, qual, key, PropertyDisambiguation::Default)
    }

    pub fn resolve_property_with_disambiguation(&mut self, base: &Symbol, qual: Option<Symbol>, key: SemanticPropertyKey, disamb: PropertyDisambiguation) -> Result<Option<Symbol>, PropertyResolutionError> {
        // 1. If base is a value whose type is one of { XML, XMLList }, return XmlReferenceValue(base, qual, key).
        if base.is_value() && [self.0.xml_type(), self.0.xml_list_type()].contains(&base.static_type(self.0)) {
            let k = key.symbol(self.0);
            return Ok(Some(self.0.factory().create_xml_reference_value(base, qual, &k, disamb)));
        }

        // 2. If base is a scope, return ResolveScopeProperty(base, qual, key).
        if base.is_scope() {
            return self.resolve_scope_property(base, qual, key, disamb);
        }

        let string_key = key.string_value();
        let number_key = key.number_value();

        // 3. If base is a value whose type is * or if key is not a String or Number constant
        //     1. Return DynamicReferenceValue(base, qual, key)
        if (base.is_value() && base.static_type(self.0) == self.0.any_type()) || !(string_key.is_some() || number_key.is_some()) {
            let k = key.symbol(self.0);
            return Ok(Some(self.0.factory().create_dynamic_reference_value(base, qual, &k, disamb)));
        }

        // 4. Return undefined if qual is not undefined.
        if qual.is_some() {
            return Ok(None);
        }

        // 5. If base is a class or enum
        if base.is_class_type() || base.is_enum_type() {
            // Key must be a String constant
            let Some(key) = string_key else {
                return Ok(None);
            };

            for class in base.descending_class_hierarchy(self.0).collect::<Vec<_>>() {
                // Throw if unresolved
                class.throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

                let r = class.static_properties(self.0).get(&key);
                if let Some(r) = r {
                    // Throw if unresolved
                    r.property_static_type(self.0).throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

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
                return Err(PropertyResolutionError::NullableBase {
                    nullable_type: base_type,
                });
            }

            // 6.2. If key is a String constant and disambiguation is one of { default, fixed }
            if [PropertyDisambiguation::Default, PropertyDisambiguation::Fixed].contains(&disamb) {
                if let Some(key) = string_key {
                    if base_type.is_class_type() || base_type.is_enum_type() {
                        for class in base_type.descending_class_hierarchy(self.0).collect::<Vec<_>>() {
                            // Throw if unresolved
                            class.throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

                            let prop = class.prototype(self.0).get(&key);
                            if let Some(prop) = prop {
                                // Throw if unresolved
                                prop.property_static_type(self.0).throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

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
                                prop.property_static_type(self.0).throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

                                return Ok(Some(self.0.factory().create_instance_reference_value(&base, &prop)));
                            }
                        }
                    }
                }
            }

            // 6.3. If disambiguation is one of { default, dynamic }
            // 6.3.1 For each descending type in the type hierarchy of base
            if [PropertyDisambiguation::Default, PropertyDisambiguation::Dynamic].contains(&disamb)
            && (base_type.is_class_type() || base_type.is_enum_type()) {
                let proxy = base_type.find_proxy(ProxyKind::GetProperty, self.0).map_err(|_| PropertyResolutionError::DeferVerification)?;
                if let Some(proxy) = proxy {
                    let key_type = key.static_type(self.0);
                    let proxy_signature = proxy.signature(self.0);

                    // Throw if proxy signature is unresolved
                    proxy_signature.throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

                    let proxy_key_type = proxy_signature.parameters().get(0).unwrap().static_type.clone();
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
            let Some(key) = string_key else {
                return Ok(None);
            };

            let r = base.properties(self.0).get(&key);
            if let Some(r) = r {
                // Throw if unresolved
                r.property_static_type(self.0).throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

                return Ok(Some(r.resolve_alias().wrap_property_reference(self.0)));
            }
            for p in base.redirect_packages().iter() {
                let r = self.resolve_property_with_disambiguation(&p, qual.clone(), SemanticPropertyKey::String(key.clone()), disamb)?;
                if r.is_some() {
                    return Ok(r);
                }
            }
            return Ok(None);
        }

        // 8. If base is a package set
        if base.is_package_set() {
            // Key must be a String constant
            let Some(key) = string_key else {
                return Ok(None);
            };

            for p in base.packages().iter() {
                let r = self.resolve_property_with_disambiguation(&p, qual.clone(), SemanticPropertyKey::String(key.clone()), disamb)?;
                if r.is_some() {
                    return Ok(r);
                }
            }
            return Ok(None);
        }

        // 9. If base is the import.meta symbol
        if base.is_import_meta() {
            // Key must be a String constant
            let Some(key) = string_key else {
                return Ok(None);
            };

            match key.as_ref() {
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
            // Key must be a String constant
            let Some(key) = string_key else {
                return Ok(None);
            };

            let ev_dict = self.0.preload_environment_variables();
            if let Some(ev) = ev_dict.get(&key) {
                let string_type = self.0.string_type();
                return Ok(Some(self.0.factory().create_string_constant(ev.clone(), &string_type)));
            } else {
                return Ok(None);
            }
        }

        // 11. Return undefined
        return Ok(None);
    }

    pub fn resolve_scope_property(&mut self, base: &Symbol, qual: Option<Symbol>, key: SemanticPropertyKey, disamb: PropertyDisambiguation) -> Result<Option<Symbol>, PropertyResolutionError> {
        // 1. If base is a with scope
        if base.is_with_scope() {
            let obj = base.object();
            let obj_static_type = obj.static_type(self.0);
            if [self.0.any_type(), self.0.xml_type(), self.0.xml_list_type()].contains(&obj_static_type) {
                let k = key.symbol(self.0);
                return Ok(Some(self.0.factory().create_dynamic_scope_reference_value(base, qual, &k, disamb)));
            }
            let r = self.resolve_property_with_disambiguation(&obj, qual.clone(), key.clone(), disamb)?;
            if let Some(r) = r {
                return Ok(Some(r));
            }
        }

        // 2. If base is a filter operator scope
        if base.is_filter_operator_scope() {
            let k = key.symbol(self.0);
            return Ok(Some(self.0.factory().create_dynamic_scope_reference_value(base, qual, &k, disamb)));
        }

        let string_key = key.string_value();

        // 3. Let r be undefined.
        let mut r: Option<Symbol> = None;

        // 4. If qual is undefined and key is a String constant
        if qual.is_none() && string_key.is_some() {
            r = base.properties(self.0).get(&string_key.clone().unwrap());
        }

        // 5. If r is not undefined
        if r.is_some() {
            // Throw if static type is unresolved
            r.clone().unwrap().property_static_type(self.0).throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

            r = Some(r.unwrap().resolve_alias().wrap_property_reference(self.0));
        }

        // 6. If base is an activation scope and base[[This]] is not undefined
        if base.is_activation_scope() && base.this().is_some() {
            r = self.resolve_property_with_disambiguation(&base.this().unwrap(), qual.clone(), key.clone(), disamb)?;
            if r.is_some() {
                return Ok(r);
            }
        }

        // 7. If base is a class scope or enum scope
        if base.is_class_scope() || base.is_enum_scope() {
            r = self.resolve_property_with_disambiguation(&base.class(), qual.clone(), key.clone(), disamb)?;
        }

        // 8. Let amb be undefined.
        let mut amb: Option<Symbol>;

        // 9. If base is a package scope
        if base.is_package_scope() {
            amb = self.resolve_property_with_disambiguation(&base.package(), qual.clone(), key.clone(), disamb)?;
            if r.is_some() {
                return Err(PropertyResolutionError::AmbiguousReference { name: string_key.clone().unwrap() });
            }
            r = amb;
        }

        // 10. If qual is undefined and key is a String constant
        if qual.is_none() && string_key.is_some() {
            let p = base.imports().get(&string_key.clone().unwrap());
            if let Some(p) = p {
                // Throw if static type is unresolved
                p.property_static_type(self.0).throw_if_unresolved().map_err(|_| PropertyResolutionError::DeferVerification)?;

                amb = Some(p.resolve_alias().wrap_property_reference(self.0));
                if r.is_some() {
                    return Err(PropertyResolutionError::AmbiguousReference { name: string_key.clone().unwrap() });
                }
                r = amb;
            }
        }

        // 11. For each op in base[[OpenPackages]]
        for p in base.packages().iter() {
            amb = self.resolve_property_with_disambiguation(&p, qual.clone(), key.clone(), disamb)?;
            if r.is_some() {
                return Err(PropertyResolutionError::AmbiguousReference { name: string_key.clone().unwrap() });
            }
            r = amb;
        }

        // 12. If r is undefined and base[[ParentScope]] is not undefined
        let parent_scope = base.parent_scope();
        if r.is_none() && parent_scope.is_some() {
            return self.resolve_scope_property(&parent_scope.unwrap(), qual, key, disamb);
        }

        // 13. Return r
        Ok(r)
    }
}