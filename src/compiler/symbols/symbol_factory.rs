use crate::ns::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct SymbolFactory<'a> {
    pub(crate) host: &'a mut SymbolHost,
}

impl<'a> SymbolFactory<'a> {
    pub fn create_unresolved(&self) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Unresolved(Cell::new(0))))
    }

    pub fn create_any_type(&self) -> Symbol {
        self.host.any_type()
    }

    pub fn create_void_type(&self) -> Symbol {
        self.host.void_type()
    }

    pub fn create_class_type(&self, name: String) -> Symbol {
        Symbol(self.host.arena.allocate(SymbolKind::Type(TypeKind::ClassType(Rc::new(ClassTypeData {
            name,
            visibility: Cell::new(Visibility::Internal),
            parent_definition: RefCell::new(None),
            super_class: RefCell::new(None),
            implements: SharedArray::new(),
            flags: Cell::new(ClassTypeFlags::empty()),
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
            super_class: RefCell::new(None),
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
            super_interfaces: SharedArray::new(),
            type_parameters: RefCell::new(None),
            prototype: SharedMap::new(),
            limited_implementors: SharedArray::new(),
            plain_metadata: SharedArray::new(),
            jetdoc: RefCell::new(None),
        })))))
    }

    pub fn create_function_type(&mut self, parameters: Vec<Rc<FunctionTypeParameter>>, result_type: Symbol) -> Symbol {
        let parameter_count = parameters.len();
        let mut collection = self.host.function_types.get_mut(&parameter_count);
        let mut empty_collection = vec![];
        if collection.is_none() {
            collection = Some(&mut empty_collection);
            self.host.function_types.insert(parameters.len(), vec![]);
        }
        for ft in collection.unwrap() {
            if result_type != ft.result_type() {
                continue;
            }
            let mut parameters_1 = parameters.iter();
            let parameters_2 = ft.parameters();
            let mut parameters_2 = parameters_2.iter();
            while let Some(param_1) = parameters_1.next() {
                let param_2 = parameters_2.next().unwrap();
                if !(param_1.kind == param_2.kind && param_1.name == param_2.name && param_1.static_type == param_2.static_type) {
                    continue;
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

    pub fn create_tuple_type(&mut self, element_types: Vec<Symbol>) -> Symbol {
        let element_count = element_types.len();
        let mut collection = self.host.tuple_types.get_mut(&element_count);
        let mut empty_collection = vec![];
        if collection.is_none() {
            collection = Some(&mut empty_collection);
            self.host.tuple_types.insert(element_count, vec![]);
        }
        for tt in collection.unwrap() {
            let mut element_types_1 = element_types.iter();
            let element_types_2 = tt.element_types();
            let mut element_types_2 = element_types_2.iter();
            while let Some(e_1) = element_types_1.next() {
                let e_2 = element_types_2.next().unwrap();
                if e_1 != &e_2 {
                    continue;
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
}