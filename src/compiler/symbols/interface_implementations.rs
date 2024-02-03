use crate::ns::*;

pub struct InterfaceImplementations<'a>(pub &'a SymbolHost);

impl<'a> InterfaceImplementations<'a> {
    pub fn verify(&mut self, implementor: &Symbol, interface: &Symbol) -> Result<Vec<InterfaceImplementationLog>, DeferVerificationError> {
        let at_package = implementor.parent_definition().unwrap().is_package();
        let expected_visibility = if at_package { Visibility::Public } else { Visibility::Internal };

        let mut interfaces = interface.all_ascending_types(self.0);
        interfaces.push(interface.clone());

        let mut log: Vec<InterfaceImplementationLog> = vec![];

        for interface in interfaces {
            interface.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

            for (name, item) in interface.prototype(self.0).borrow().iter() {
                let implementor_item = implementor.prototype(self.0).get(name);

                if implementor_item.is_some() && implementor_item.clone().unwrap().visibility() != expected_visibility {
                    log.push(InterfaceImplementationLog::WrongVisibility { name: name.clone(), expected_visibility });
                }

                if implementor_item.is_none() {
                    if item.is_virtual_property() {
                        if item.getter(self.0).is_some() && !item.getter(self.0).unwrap().is_optional_interface_method() {
                            log.push(InterfaceImplementationLog::UnimplementedGetter { name: name.clone() });
                        }
                        if item.setter(self.0).is_some() && !item.setter(self.0).unwrap().is_optional_interface_method() {
                            log.push(InterfaceImplementationLog::UnimplementedSetter { name: name.clone() });
                        }
                    } else if !item.is_optional_interface_method() {
                        log.push(InterfaceImplementationLog::UnimplementedMethod { name: name.clone() });
                    }
                // Verify accessors
                } else if item.is_virtual_property() {
                    let implementor_item = implementor_item.unwrap();
                    if !implementor_item.is_virtual_property() {
                        log.push(InterfaceImplementationLog::PropertyMustBeVirtualProperty { name: name.clone() });
                    } else {
                        // Getter
                        if implementor_item.getter(self.0).is_none() {
                            if item.getter(self.0).is_some() && !item.getter(self.0).unwrap().is_optional_interface_method() {
                                log.push(InterfaceImplementationLog::UnimplementedGetter { name: name.clone() });
                            }
                        } else if item.getter(self.0).is_some() && item.getter(self.0).unwrap().signature(self.0) != implementor_item.getter(self.0).unwrap().signature(self.0) {
                            let expected_signature = item.getter(self.0).unwrap().signature(self.0);
                            expected_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

                            let actual_signature = implementor_item.getter(self.0).unwrap().signature(self.0);
                            actual_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

                            log.push(InterfaceImplementationLog::WrongGetterSignature {
                                name: name.clone(), expected_signature,
                            });
                        }

                        // Setter
                        if implementor_item.setter(self.0).is_none() {
                            if item.setter(self.0).is_some() && !item.setter(self.0).unwrap().is_optional_interface_method() {
                                log.push(InterfaceImplementationLog::UnimplementedSetter { name: name.clone() });
                            }
                        } else if item.setter(self.0).is_some() && item.setter(self.0).unwrap().signature(self.0) != implementor_item.setter(self.0).unwrap().signature(self.0) {
                            let expected_signature = item.setter(self.0).unwrap().signature(self.0);
                            expected_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

                            let actual_signature = implementor_item.setter(self.0).unwrap().signature(self.0);
                            actual_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

                            log.push(InterfaceImplementationLog::WrongSetterSignature {
                                name: name.clone(), expected_signature,
                            });
                        }
                    }
                // Verify regular method
                } else {
                    let implementor_item = implementor_item.unwrap();
                    if !implementor_item.is_function() {
                        log.push(InterfaceImplementationLog::PropertyMustBeMethod { name: name.clone() });
                    }

                    if item.type_parameters().is_some() {
                        // Type parameterized method
                        if self.conforming_type_parameters(&item.type_parameters().unwrap(), implementor_item.type_parameters()) {
                            let expected_signature = item.signature(self.0).type_substitution(self.0, &item.type_parameters().unwrap(), &implementor_item.type_parameters().unwrap());
                            expected_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;
 
                            let actual_signature = implementor_item.signature(self.0);
                            actual_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

                            if expected_signature != actual_signature {
                                log.push(InterfaceImplementationLog::WrongMethodSignature {
                                    name: name.clone(), expected_signature,
                                });
                            }
                        } else {
                            log.push(InterfaceImplementationLog::NonConformingTypeParameters { name: name.clone() });
                        }
                    } else if implementor_item.type_parameters().is_some() {
                        log.push(InterfaceImplementationLog::NonConformingTypeParameters { name: name.clone() });
                    } else {
                        let expected_signature = item.signature(self.0);
                        expected_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;
 
                        let actual_signature = implementor_item.signature(self.0);
                        actual_signature.throw_if_unresolved().map_err(|_| DeferVerificationError)?;

                        if expected_signature != actual_signature {
                            log.push(InterfaceImplementationLog::WrongMethodSignature {
                                name: name.clone(), expected_signature,
                            });
                        }
                    }
                }
            }
        }

        Ok(log)
    }

    fn conforming_type_parameters(&mut self, type_parameters_1: &SharedArray<Symbol>, type_parameters_2: Option<SharedArray<Symbol>>) -> bool {
        if type_parameters_2.is_none() {
            return false;
        }
        let type_parameters_2 = type_parameters_2.unwrap();
        type_parameters_1.length() == type_parameters_2.length()
    }
}

pub enum InterfaceImplementationLog {
    UnimplementedMethod { name: String },
    UnimplementedGetter { name: String },
    UnimplementedSetter { name: String },
    PropertyMustBeMethod { name: String },
    PropertyMustBeVirtualProperty { name: String },
    WrongMethodSignature { name: String, expected_signature: Symbol },
    WrongGetterSignature { name: String, expected_signature: Symbol },
    WrongSetterSignature { name: String, expected_signature: Symbol },
    WrongVisibility { name: String, expected_visibility: Visibility },
    NonConformingTypeParameters { name: String },
}