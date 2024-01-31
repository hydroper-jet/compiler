use crate::ns::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct SymbolFactory<'a> {
    pub(crate) host: &'a mut SymbolHost,
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
            list_of_to_proxies: SharedMap::new(),
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
            list_of_to_proxies: SharedMap::new(),
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
    pub fn create_function_type(&mut self, parameters: Vec<Rc<FunctionTypeParameter>>, result_type: Symbol) -> Symbol {
        let parameter_count = parameters.len();
        let mut collection = self.host.function_types.get_mut(&parameter_count);
        let mut empty_collection = vec![];
        if collection.is_none() {
            collection = Some(&mut empty_collection);
            self.host.function_types.insert(parameters.len(), vec![]);
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

        let collection = self.host.function_types.get_mut(&parameter_count);
        collection.unwrap().push(ft.clone());

        ft
    }

    /// Creates an interned tuple type.
    pub fn create_tuple_type(&mut self, element_types: Vec<Symbol>) -> Symbol {
        let element_count = element_types.len();
        let mut collection = self.host.tuple_types.get_mut(&element_count);
        let mut empty_collection = vec![];
        if collection.is_none() {
            collection = Some(&mut empty_collection);
            self.host.tuple_types.insert(element_count, vec![]);
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

        let collection = self.host.tuple_types.get_mut(&element_count);
        collection.unwrap().push(tt.clone());

        tt
    }

    /// Creates an interned nullable type.
    pub fn create_nullable_type(&mut self, base: &Symbol) -> Symbol {
        if base.includes_null() {
            return base.clone();
        }
        let nt = self.host.nullable_types.get(base);
        if let Some(nt) = nt {
            return nt.clone();
        }
        let nt = Symbol(self.host.arena.allocate(SymbolKind::Type(TypeKind::NullableType(base.clone()))));
        self.host.nullable_types.insert(base.clone(), nt.clone());
        nt
    }

    pub fn create_type_parameter_type(&self, name: String) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Type(TypeKind::TypeParameterType(Rc::new(TypeParameterTypeData {
            name,
        })))))
    }

    /// Creates an interned type after explicit type substitution.
    pub fn create_type_after_explicit_type_substitution(&mut self, origin: &Symbol, substitute_types: &SharedArray<Symbol>) -> Symbol {
        // Verify parameter count
        let parameters = origin.type_parameters().unwrap();
        let parameter_count = parameters.length();
        assert_eq!(substitute_types.length(), parameter_count);

        let mut list = self.host.taets.get(&origin);
        let empty_list = vec![];
        if list.is_none() {
            list = Some(&empty_list);
            self.host.taets.insert(origin.clone(), vec![]);
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

        let list = self.host.taets.get_mut(&origin).unwrap();
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

    pub fn create_variable_property_after_indirect_type_substitution(&mut self, origin: &Symbol, indirect_type_parameters: &SharedArray<Symbol>, indirect_substitute_types: &SharedArray<Symbol>) -> Symbol {
        // Verify parameter count
        assert_eq!(indirect_type_parameters.length(), indirect_substitute_types.length());

        let mut list = self.host.vpaits.get(indirect_type_parameters);
        let empty_list = vec![];
        if list.is_none() {
            list = Some(&empty_list);
            self.host.vpaits.insert(indirect_type_parameters.clone(), vec![]);
        }
        'vpaits: for vpaits in list.unwrap() {
            let mut substitute_types_1 = indirect_substitute_types.iter();
            let substitute_types_2 = vpaits.indirect_substitute_types();
            let mut substitute_types_2 = substitute_types_2.iter();
            while let Some(substitute_type_1) = substitute_types_1.next() {
                let substitute_type_2 = substitute_types_2.next().unwrap();
                if substitute_type_1 != substitute_type_2 {
                    continue 'vpaits;
                }
            }
            return vpaits.clone();
        }

        let vpaits = Symbol(self.host.arena.allocate(SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(Rc::new(VariablePropertyAfterIndirectTypeSubstitutionData {
            origin: origin.clone(),
            indirect_type_parameters: indirect_type_parameters.clone(),
            indirect_substitute_types: indirect_substitute_types.clone(),
            static_type: RefCell::new(None),
        }))));

        let list = self.host.vpaits.get_mut(&indirect_type_parameters).unwrap();
        list.push(vpaits.clone());

        vpaits
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
}