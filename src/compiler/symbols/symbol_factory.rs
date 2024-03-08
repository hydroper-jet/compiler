use crate::ns::*;

pub struct SymbolFactory<'a> {
    pub(crate) host: &'a SymbolHost,
}

impl<'a> SymbolFactory<'a> {
    /// Returns the unique `Unresolved` symbol.
    pub fn create_unresolved(&self) -> Symbol {
        self.host.unresolved()
    }

    /// Returns the unique `AnyType` symbol.
    pub fn create_any_type(&self) -> Symbol {
        self.host.any_type()
    }

    /// Returns the unique `VoidType` symbol.
    pub fn create_void_type(&self) -> Symbol {
        self.host.void_type()
    }

    pub fn create_class_type(&self, name: String) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Type(TypeKind::ClassType(Rc::new(ClassTypeData {
            name,
            visibility: Cell::new(Visibility::Internal),
            parent_definition: RefCell::new(None),
            extends_class: RefCell::new(None),
            implements: SharedArray::new(),
            flags: RefCell::new(ClassTypeFlags::empty()),
            type_parameters: RefCell::new(None),
            static_properties: SharedMap::new(),
            constructor_function: RefCell::new(None),
            prototype: SharedMap::new(),
            proxies: SharedMap::new(),
            list_of_to_proxies: SharedArray::new(),
            limited_subclasses: SharedArray::new(),
            plain_metadata: SharedArray::new(),
            jetdoc: RefCell::new(None),
        })))))
    }

    pub fn create_enum_type(&self, name: String, is_set_enumeration: bool) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Type(TypeKind::EnumType(Rc::new(EnumTypeData {
            name,
            visibility: Cell::new(Visibility::Internal),
            parent_definition: RefCell::new(None),
            representation_type: RefCell::new(None),
            is_set_enumeration,
            static_properties: SharedMap::new(),
            prototype: SharedMap::new(),
            proxies: SharedMap::new(),
            list_of_to_proxies: SharedArray::new(),
            enumeration_members: SharedMap::new(),
            plain_metadata: SharedArray::new(),
            jetdoc: RefCell::new(None),
        })))))
    }

    pub fn create_interface_type(&self, name: String) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Type(TypeKind::InterfaceType(Rc::new(InterfaceTypeData {
            name,
            visibility: Cell::new(Visibility::Internal),
            parent_definition: RefCell::new(None),
            extends_interfaces: SharedArray::new(),
            type_parameters: RefCell::new(None),
            prototype: SharedMap::new(),
            limited_implementors: SharedArray::new(),
            plain_metadata: SharedArray::new(),
            jetdoc: RefCell::new(None),
        })))))
    }

    /// Creates an interned function type.
    pub fn create_function_type(&self, parameters: Vec<Rc<ParameterOfFunctionType>>, result_type: Symbol) -> Symbol {
        let parameter_count = parameters.len();
        let mut function_types = self.host.function_types.borrow_mut();
        let mut collection = function_types.get_mut(&parameter_count);
        let mut empty_collection = vec![];
        if collection.is_none() {
            collection = Some(&mut empty_collection);
            function_types.insert(parameters.len(), vec![]);
        }
        'ft: for ft in collection.unwrap() {
            if result_type != ft.result_type() {
                continue 'ft;
            }
            let mut parameters_1 = parameters.iter();
            let parameters_2 = ft.parameters();
            let mut parameters_2 = parameters_2.iter();
            while let Some(param_1) = parameters_1.next() {
                let param_2 = parameters_2.next().unwrap();
                if !(param_1.kind == param_2.kind && param_1.name == param_2.name && param_1.static_type == param_2.static_type) {
                    continue 'ft;
                }
            }
            return ft.clone();
        }
        let ft = Symbol(self.host.arena.allocate(SymbolKind::Type(TypeKind::FunctionType(Rc::new(FunctionTypeData {
            parameters: SharedArray::from(parameters),
            result_type,
        })))));

        let collection = function_types.get_mut(&parameter_count);
        collection.unwrap().push(ft.clone());

        ft
    }

    /// Creates an interned tuple type.
    pub fn create_tuple_type(&self, element_types: Vec<Symbol>) -> Symbol {
        let element_count = element_types.len();
        let mut tuple_types = self.host.tuple_types.borrow_mut();
        let mut collection = tuple_types.get_mut(&element_count);
        let mut empty_collection = vec![];
        if collection.is_none() {
            collection = Some(&mut empty_collection);
            tuple_types.insert(element_count, vec![]);
        }
        'tt: for tt in collection.unwrap() {
            let mut element_types_1 = element_types.iter();
            let element_types_2 = tt.element_types();
            let mut element_types_2 = element_types_2.iter();
            while let Some(e_1) = element_types_1.next() {
                let e_2 = element_types_2.next().unwrap();
                if e_1 != &e_2 {
                    continue 'tt;
                }
            }
            return tt.clone();
        }
        let tt = Symbol(self.host.arena.allocate(SymbolKind::Type(TypeKind::TupleType(Rc::new(TupleTypeData {
            element_types: SharedArray::from(element_types),
        })))));

        let collection = tuple_types.get_mut(&element_count);
        collection.unwrap().push(tt.clone());

        tt
    }

    /// Creates an interned nullable type. Returns `base` back
    /// if it already includes `null`.
    pub fn create_nullable_type(&self, base: &Symbol) -> Symbol {
        if base.includes_null() {
            return base.clone();
        }
        let mut nullable_types = self.host.nullable_types.borrow_mut();
        let nt = nullable_types.get(base);
        if let Some(nt) = nt {
            return nt.clone();
        }
        let nt = Symbol(self.host.arena.allocate(SymbolKind::Type(TypeKind::NullableType(base.clone()))));
        nullable_types.insert(base.clone(), nt.clone());
        nt
    }

    pub fn create_type_parameter_type(&self, name: String) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Type(TypeKind::TypeParameterType(Rc::new(TypeParameterTypeData {
            name,
        })))))
    }

    /// Creates an interned type after explicit type substitution.
    pub fn create_type_after_explicit_type_substitution(&self, origin: &Symbol, substitute_types: &SharedArray<Symbol>) -> Symbol {
        // Verify parameter count
        let parameters = origin.type_parameters().unwrap();
        let parameter_count = parameters.length();
        assert_eq!(substitute_types.length(), parameter_count);

        let mut taets_list = self.host.taets.borrow_mut();

        let mut list = taets_list.get(&origin);
        let empty_list = vec![];
        if list.is_none() {
            list = Some(&empty_list);
            taets_list.insert(origin.clone(), vec![]);
        }
        'taets: for taets in list.unwrap() {
            let mut substitute_types_1 = substitute_types.iter();
            let substitute_types_2 = taets.substitute_types();
            let mut substitute_types_2 = substitute_types_2.iter();
            while let Some(substitute_type_1) = substitute_types_1.next() {
                let substitute_type_2 = substitute_types_2.next().unwrap();
                if substitute_type_1 != substitute_type_2 {
                    continue 'taets;
                }
            }
            return taets.clone();
        }

        let taets = Symbol(self.host.arena.allocate(SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(Rc::new(TypeAfterExplicitTypeSubstitutionData {
            origin: origin.clone(),
            substitute_types: substitute_types.clone(),
            extends_class: RefCell::new(None),
            implements: RefCell::new(None),
            extends_interfaces: RefCell::new(None),
            static_properties: RefCell::new(None),
            constructor_function: RefCell::new(None),
            prototype: RefCell::new(None),
            proxies: RefCell::new(None),
            list_of_to_proxies: RefCell::new(None),
        })))));

        let list = taets_list.get_mut(&origin).unwrap();
        list.push(taets.clone());

        taets
    }

    pub fn create_alias(&self, name: String, alias_of: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Alias(Rc::new(AliasData {
            name,
            visibility: Cell::new(Visibility::Internal),
            alias_of: RefCell::new(alias_of.clone()),
            parent_definition: RefCell::new(None),
            plain_metadata: SharedArray::new(),
            jetdoc: RefCell::new(None),
        }))))
    }

    /// Creates an interned package from a fully qualified name.
    ///
    /// # Example
    ///
    /// ```ignore
    /// assert_eq!(host.factory.create_package(["q", "b", "w"]).fully_qualified_name(), "q.b.w");
    /// ```
    pub fn create_package<'b>(&self, name: impl IntoIterator<Item = &'b str>) -> Symbol {
        self.create_package_1(&name.into_iter().collect())
    }

    fn create_package_1(&self, name: &Vec<&str>) -> Symbol {
        let mut result: Symbol = self.host.top_level_package.clone();
        for name_1 in name {
            let name_1 = (*name_1).to_owned();
            let result_1 = result.subpackages().get(&name_1);
            if let Some(result_1) = result_1 {
                result = result_1;
            } else {
                let result_1 = Symbol(self.host.arena.allocate(SymbolKind::Package(Rc::new(PackageData {
                    name: name_1.clone(),
                    parent_definition: RefCell::new(Some(result.clone())),
                    properties: SharedMap::new(),
                    redirect_packages: SharedArray::new(),
                    subpackages: SharedMap::new(),
                    jetdoc: RefCell::new(None),
                }))));
                result.subpackages().set(name_1, result_1.clone());
                result = result_1;
            }
        }
        result
    }

    pub fn create_package_set(&self, name: String, packages: SharedArray<Symbol>) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::PackageSet(Rc::new(PackageSetData {
            name,
            parent_definition: RefCell::new(None),
            packages,
            visibility: Cell::new(Visibility::Internal),
            jetdoc: RefCell::new(None),
        }))))
    }

    pub fn create_variable_property(&self, name: String, read_only: bool, static_type: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::VariableProperty(Rc::new(VariablePropertyData {
            name,
            visibility: Cell::new(Visibility::Internal),
            read_only: Cell::new(read_only),
            static_type: RefCell::new(static_type.clone()),
            constant_initializer: RefCell::new(None),
            parent_definition: RefCell::new(None),
            plain_metadata: SharedArray::new(),
            jetdoc: RefCell::new(None),
        }))))
    }

    /// Creates an interned variable property after indirect type substitution.
    pub fn create_variable_property_after_indirect_type_substitution(&self, origin: &Symbol, indirect_type_parameters: &SharedArray<Symbol>, indirect_substitute_types: &SharedArray<Symbol>) -> Symbol {
        // Verify parameter count
        assert_eq!(indirect_type_parameters.length(), indirect_substitute_types.length());

        let mut vapaits_list = self.host.vapaits.borrow_mut();

        let mut base_list = vapaits_list.get_mut(origin);
        let mut empty_base_list = HashMap::<SharedArray<Symbol>, Vec<Symbol>>::new();
        if base_list.is_none() {
            base_list = Some(&mut empty_base_list);
            vapaits_list.insert(origin.clone(), HashMap::new());
        }
        let base_list = base_list.unwrap();

        let mut list = base_list.get(indirect_type_parameters);
        let empty_list = vec![];
        if list.is_none() {
            list = Some(&empty_list);
            base_list.insert(indirect_type_parameters.clone(), vec![]);
        }
        'vapaits: for vapaits in list.unwrap() {
            let mut substitute_types_1 = indirect_substitute_types.iter();
            let substitute_types_2 = vapaits.indirect_substitute_types();
            let mut substitute_types_2 = substitute_types_2.iter();
            while let Some(substitute_type_1) = substitute_types_1.next() {
                let substitute_type_2 = substitute_types_2.next().unwrap();
                if substitute_type_1 != substitute_type_2 {
                    continue 'vapaits;
                }
            }
            return vapaits.clone();
        }

        let vapaits = Symbol(self.host.arena.allocate(SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(Rc::new(VariablePropertyAfterIndirectTypeSubstitutionData {
            origin: origin.clone(),
            indirect_type_parameters: indirect_type_parameters.clone(),
            indirect_substitute_types: indirect_substitute_types.clone(),
            static_type: RefCell::new(None),
        }))));

        let list = vapaits_list.get_mut(origin).unwrap().get_mut(&indirect_type_parameters).unwrap();
        list.push(vapaits.clone());

        vapaits
    }

    pub fn create_virtual_property(&self, name: String) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::VirtualProperty(Rc::new(VirtualPropertyData {
            name,
            visibility: Cell::new(Visibility::Internal),
            static_type: RefCell::new(None),
            getter: RefCell::new(None),
            setter: RefCell::new(None),
            parent_definition: RefCell::new(None),
            jetdoc: RefCell::new(None),
        }))))
    }

    /// Creates an interned virtual property after indirect type substitution.
    pub fn create_virtual_property_after_indirect_type_substitution(&self, origin: &Symbol, indirect_type_parameters: &SharedArray<Symbol>, indirect_substitute_types: &SharedArray<Symbol>) -> Symbol {
        // Verify parameter count
        assert_eq!(indirect_type_parameters.length(), indirect_substitute_types.length());

        let mut vipaits_list = self.host.vipaits.borrow_mut();

        let mut base_list = vipaits_list.get_mut(origin);
        let mut empty_base_list = HashMap::<SharedArray<Symbol>, Vec<Symbol>>::new();
        if base_list.is_none() {
            base_list = Some(&mut empty_base_list);
            vipaits_list.insert(origin.clone(), HashMap::new());
        }
        let base_list = base_list.unwrap();

        let mut list = base_list.get(indirect_type_parameters);
        let empty_list = vec![];
        if list.is_none() {
            list = Some(&empty_list);
            base_list.insert(indirect_type_parameters.clone(), vec![]);
        }
        'vipaits: for vipaits in list.unwrap() {
            let mut substitute_types_1 = indirect_substitute_types.iter();
            let substitute_types_2 = vipaits.indirect_substitute_types();
            let mut substitute_types_2 = substitute_types_2.iter();
            while let Some(substitute_type_1) = substitute_types_1.next() {
                let substitute_type_2 = substitute_types_2.next().unwrap();
                if substitute_type_1 != substitute_type_2 {
                    continue 'vipaits;
                }
            }
            return vipaits.clone();
        }

        let vipaits = Symbol(self.host.arena.allocate(SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(Rc::new(VirtualPropertyAfterIndirectTypeSubstitutionData {
            origin: origin.clone(),
            indirect_type_parameters: indirect_type_parameters.clone(),
            indirect_substitute_types: indirect_substitute_types.clone(),
            static_type: RefCell::new(None),
            getter: RefCell::new(None),
            setter: RefCell::new(None),
        }))));

        let list = vipaits_list.get_mut(origin).unwrap().get_mut(&indirect_type_parameters).unwrap();
        list.push(vipaits.clone());

        vipaits
    }

    pub fn create_function(&self, name: String, signature: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Function(Rc::new(FunctionSymbolData {
            name,
            visibility: Cell::new(Visibility::Internal),
            parent_definition: RefCell::new(None),
            plain_metadata: SharedArray::new(),
            jetdoc: RefCell::new(None),
            flags: RefCell::new(FunctionSymbolFlags::empty()),
            signature: RefCell::new(signature.clone()),
            type_parameters: RefCell::new(None),
            of_virtual_property: RefCell::new(None),
            overriden_by: SharedArray::new(),
            overrides_method: RefCell::new(None),
            activation_scope: RefCell::new(None),
        }))))
    }

    /// Creates an interned function after explicit or indirect type substitution.
    pub fn create_function_after_explicit_or_indirect_type_substitution(&self, origin: &Symbol, explicit_or_indirect_type_parameters: &SharedArray<Symbol>, explicit_or_indirect_substitute_types: &SharedArray<Symbol>) -> Symbol {
        // Verify parameter count
        assert_eq!(explicit_or_indirect_type_parameters.length(), explicit_or_indirect_substitute_types.length());

        let mut faeoits_list = self.host.faeoits.borrow_mut();

        let mut base_list = faeoits_list.get_mut(origin);
        let mut empty_base_list = HashMap::<SharedArray<Symbol>, Vec<Symbol>>::new();
        if base_list.is_none() {
            base_list = Some(&mut empty_base_list);
            faeoits_list.insert(origin.clone(), HashMap::new());
        }
        let base_list = base_list.unwrap();

        let mut list = base_list.get(explicit_or_indirect_type_parameters);
        let empty_list = vec![];
        if list.is_none() {
            list = Some(&empty_list);
            base_list.insert(explicit_or_indirect_type_parameters.clone(), vec![]);
        }
        'faeoits: for faeoits in list.unwrap() {
            let mut substitute_types_1 = explicit_or_indirect_substitute_types.iter();
            let substitute_types_2 = faeoits.explicit_or_indirect_substitute_types();
            let mut substitute_types_2 = substitute_types_2.iter();
            while let Some(substitute_type_1) = substitute_types_1.next() {
                let substitute_type_2 = substitute_types_2.next().unwrap();
                if substitute_type_1 != substitute_type_2 {
                    continue 'faeoits;
                }
            }
            return faeoits.clone();
        }

        let faeoits = Symbol(self.host.arena.allocate(SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(Rc::new(FunctionAfterExplicitOrIndirectTypeSubstitutionData {
            origin: origin.clone(),
            explicit_or_indirect_type_parameters: explicit_or_indirect_type_parameters.clone(),
            explicit_or_indirect_substitute_types: explicit_or_indirect_substitute_types.clone(),
            signature: RefCell::new(None),
            of_virtual_property: RefCell::new(None),
            overriden_by: RefCell::new(None),
            overrides_method: RefCell::new(None),
            is_overriding: Cell::new(origin.is_overriding()),
        }))));

        let list = faeoits_list.get_mut(origin).unwrap().get_mut(&explicit_or_indirect_type_parameters).unwrap();
        list.push(faeoits.clone());

        faeoits
    }

    pub fn create_scope(&self) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Scope(Rc::new(ScopeData {
            parent_scope: RefCell::new(None),
            properties: SharedMap::new(),
            imports: SharedMap::new(),
            open_packages: SharedArray::new(),
            package_aliases: SharedMap::new(),
            local_variable_scope_count: Cell::new(0),
        }), None)))
    }

    pub fn create_with_scope(&self, object: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Scope(Rc::new(ScopeData {
            parent_scope: RefCell::new(None),
            properties: SharedMap::new(),
            imports: SharedMap::new(),
            open_packages: SharedArray::new(),
            package_aliases: SharedMap::new(),
            local_variable_scope_count: Cell::new(0),
        }), Some(ScopeKind::With {
            object: object.clone(),
        }))))
    }

    pub fn create_filter_operator_scope(&self, base: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Scope(Rc::new(ScopeData {
            parent_scope: RefCell::new(None),
            properties: SharedMap::new(),
            imports: SharedMap::new(),
            open_packages: SharedArray::new(),
            package_aliases: SharedMap::new(),
            local_variable_scope_count: Cell::new(0),
        }), Some(ScopeKind::FilterOperator {
            base: base.clone(),
        }))))
    }

    pub fn create_activation_scope(&self, function: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Scope(Rc::new(ScopeData {
            parent_scope: RefCell::new(None),
            properties: SharedMap::new(),
            imports: SharedMap::new(),
            open_packages: SharedArray::new(),
            package_aliases: SharedMap::new(),
            local_variable_scope_count: Cell::new(0),
        }), Some(ScopeKind::Activation(Rc::new(ActivationScopeData {
            function: function.clone(),
            this: RefCell::new(None),
            property_has_capture: RefCell::new(None),
        }))))))
    }

    pub fn create_class_scope(&self, class: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Scope(Rc::new(ScopeData {
            parent_scope: RefCell::new(None),
            properties: SharedMap::new(),
            imports: SharedMap::new(),
            open_packages: SharedArray::new(),
            package_aliases: SharedMap::new(),
            local_variable_scope_count: Cell::new(0),
        }), Some(ScopeKind::Class {
            class: class.clone(),
        }))))
    }

    pub fn create_enum_scope(&self, class: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Scope(Rc::new(ScopeData {
            parent_scope: RefCell::new(None),
            properties: SharedMap::new(),
            imports: SharedMap::new(),
            open_packages: SharedArray::new(),
            package_aliases: SharedMap::new(),
            local_variable_scope_count: Cell::new(0),
        }), Some(ScopeKind::Enum {
            class: class.clone(),
        }))))
    }

    pub fn create_interface_scope(&self, interface: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Scope(Rc::new(ScopeData {
            parent_scope: RefCell::new(None),
            properties: SharedMap::new(),
            imports: SharedMap::new(),
            open_packages: SharedArray::new(),
            package_aliases: SharedMap::new(),
            local_variable_scope_count: Cell::new(0),
        }), Some(ScopeKind::Interface {
            interface: interface.clone(),
        }))))
    }

    pub fn create_package_scope(&self, package: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Scope(Rc::new(ScopeData {
            parent_scope: RefCell::new(None),
            properties: SharedMap::new(),
            imports: SharedMap::new(),
            open_packages: SharedArray::new(),
            package_aliases: SharedMap::new(),
            local_variable_scope_count: Cell::new(0),
        }), Some(ScopeKind::Package {
            package: package.clone(),
        }))))
    }

    pub fn create_value(&self, static_type: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(static_type.clone()),
        }, None)))
    }

    pub fn create_undefined_constant(&self, static_type: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(static_type.clone()),
        }, Some(Rc::new(ValueKind::Constant(ConstantKind::Undefined))))))
    }

    pub fn create_null_constant(&self, static_type: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(static_type.clone()),
        }, Some(Rc::new(ValueKind::Constant(ConstantKind::Null))))))
    }

    pub fn create_string_constant(&self, value: String, static_type: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(static_type.clone()),
        }, Some(Rc::new(ValueKind::Constant(ConstantKind::String(value)))))))
    }

    pub fn create_char_constant(&self, value: char, static_type: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(static_type.clone()),
        }, Some(Rc::new(ValueKind::Constant(ConstantKind::Char(value)))))))
    }

    pub fn create_boolean_constant(&self, value: bool, static_type: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(static_type.clone()),
        }, Some(Rc::new(ValueKind::Constant(ConstantKind::Boolean(value)))))))
    }

    pub fn create_number_constant(&self, value: AbstractRangeNumber, static_type: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(static_type.clone()),
        }, Some(Rc::new(ValueKind::Constant(ConstantKind::Number(value)))))))
    }

    pub fn create_enum_constant(&self, value: AbstractRangeNumber, static_type: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(static_type.clone()),
        }, Some(Rc::new(ValueKind::Constant(ConstantKind::Enum(value)))))))
    }

    pub fn create_this_value(&self, static_type: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(static_type.clone()),
        }, Some(Rc::new(ValueKind::This)))))
    }

    pub fn create_conversion_value(&self, base: &Symbol, relationship: TypeConversionRelationship, optional: bool, target: &Symbol) -> Symbol {
        let st = if optional { self.create_nullable_type(target) } else { target.clone() };
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(st),
        }, Some(Rc::new(ValueKind::Conversion(Rc::new(ConversionValueData {
            base: base.clone(),
            relationship,
            optional,
            target: target.clone(),
        })))))))
    }

    pub fn create_embed_value(&self, content: EmbedValueDataContent) -> Symbol {
        match content {
            EmbedValueDataContent::String(s) => {
                let string_type = self.host.string_type();
                Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
                    static_type: RefCell::new(string_type),
                }, Some(Rc::new(ValueKind::Embed(Rc::new(EmbedValueData {
                    embedded_string: Some(Rc::new(s)),
                    embedded_byte_array: None,
                })))))))
            },
            EmbedValueDataContent::ByteArray(ba) => {
                let byte_array_type = self.host.byte_array_type();
                Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
                    static_type: RefCell::new(byte_array_type),
                }, Some(Rc::new(ValueKind::Embed(Rc::new(EmbedValueData {
                    embedded_string: None,
                    embedded_byte_array: Some(Rc::new(ba)),
                })))))))
            },
        }
    }

    pub fn create_import_meta_output_value(&self) -> Symbol {
        let string_type = self.host.string_type();
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(string_type),
        }, Some(Rc::new(ValueKind::ImportMetaOutput)))))
    }

    pub fn create_type_as_reference_value(&self, referenced_type: &Symbol) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(self.host.class_type()),
        }, Some(Rc::new(ValueKind::Reference(Rc::new(ReferenceValueKind::Type {
            referenced_type: referenced_type.clone(),
        })))))))
    }

    pub fn create_xml_reference_value(&self, base: &Symbol, qualifier: Option<Symbol>, key: &Symbol, disamb: PropertyDisambiguation) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(self.host.any_type()),
        }, Some(Rc::new(ValueKind::Reference(Rc::new(ReferenceValueKind::Xml {
            base: base.clone(),
            qualifier,
            key: key.clone(),
            disambiguation: disamb,
        })))))))
    }

    pub fn create_dynamic_reference_value(&self, base: &Symbol, qualifier: Option<Symbol>, key: &Symbol, disamb: PropertyDisambiguation) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(self.host.any_type()),
        }, Some(Rc::new(ValueKind::Reference(Rc::new(ReferenceValueKind::Dynamic {
            base: base.clone(),
            qualifier,
            key: key.clone(),
            disambiguation: disamb,
        })))))))
    }

    pub fn create_static_reference_value(&self, base: &Symbol, property: &Symbol) -> Symbol {
        let st = property.property_static_type(self.host);
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(st),
        }, Some(Rc::new(ValueKind::Reference(Rc::new(ReferenceValueKind::Static {
            base: base.clone(),
            property: property.clone(),
        })))))))
    }

    pub fn create_instance_reference_value(&self, base: &Symbol, property: &Symbol) -> Symbol {
        let st = property.property_static_type(self.host);
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(st),
        }, Some(Rc::new(ValueKind::Reference(Rc::new(ReferenceValueKind::Instance {
            base: base.clone(),
            property: property.clone(),
        })))))))
    }

    pub fn create_proxy_reference_value(&self, base: &Symbol, proxy: &Symbol) -> Symbol {
        let st = proxy.signature(self.host).result_type();
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(st),
        }, Some(Rc::new(ValueKind::Reference(Rc::new(ReferenceValueKind::Proxy {
            base: base.clone(),
            proxy: proxy.clone(),
        })))))))
    }

    pub fn create_tuple_reference_value(&self, base: &Symbol, index: usize) -> Symbol {
        let st = base.static_type(self.host).non_null_type().element_types().get(index).unwrap();
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(st),
        }, Some(Rc::new(ValueKind::Reference(Rc::new(ReferenceValueKind::Tuple {
            base: base.clone(),
            index,
        })))))))
    }

    pub fn create_scope_reference_value(&self, base: &Symbol, property: &Symbol) -> Symbol {
        let st = property.property_static_type(self.host);
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(st),
        }, Some(Rc::new(ValueKind::Reference(Rc::new(ReferenceValueKind::Scope {
            base: base.clone(),
            property: property.clone(),
        })))))))
    }

    pub fn create_dynamic_scope_reference_value(&self, base: &Symbol, qualifier: Option<Symbol>, key: &Symbol, disamb: PropertyDisambiguation) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(self.host.any_type()),
        }, Some(Rc::new(ValueKind::Reference(Rc::new(ReferenceValueKind::DynamicScope {
            base: base.clone(),
            qualifier,
            key: key.clone(),
            disambiguation: disamb,
        })))))))
    }

    pub fn create_package_reference_value(&self, base: &Symbol, property: &Symbol) -> Symbol {
        let st = property.property_static_type(self.host);
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(st),
        }, Some(Rc::new(ValueKind::Reference(Rc::new(ReferenceValueKind::Package {
            base: base.clone(),
            property: property.clone(),
        })))))))
    }

    /// Creates a function value as result of a function expression.
    ///
    /// # Panics
    ///
    /// Panics if the activation's signature is unresolved.
    pub fn create_function_value(&self, activation_scope: &Symbol) -> Symbol {
        let signature = activation_scope.function().signature(self.host);
        assert!(!signature.is_unresolved());
        Symbol(self.host.arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(signature),
        }, Some(Rc::new(ValueKind::Function {
            activation_scope: activation_scope.clone(),
        })))))
    }

    pub fn create_block_statement(&self) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::BlockStatement(Rc::new(BlockStatementSymbolData {
            plain_metadata: SharedArray::new(),
        }))))
    }

    pub fn create_variable_definition_directive(&self) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::VariableDefinitionDirective(Rc::new(VariableDefinitionDirectiveSymbolData {
            shadow_scope: RefCell::new(None),
        }))))
    }
}