use crate::ns::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct SymbolFactory<'a> {
    pub(crate) host: &'a SymbolHost,
    pub(crate) arena: &'a Arena<SymbolKind>,
}

impl<'a> SymbolFactory<'a> {
    pub fn create_unresolved(&self) -> Symbol {
        Symbol(self.arena.allocate(SymbolKind::Unresolved(Cell::new(0))))
    }

    pub fn create_any_type(&self) -> Symbol {
        self.host.any_type()
    }

    pub fn create_void_type(&self) -> Symbol {
        self.host.void_type()
    }

    pub fn create_class_type(&self, name: String) -> Symbol {
        Symbol(self.arena.allocate(SymbolKind::Type(TypeKind::ClassType(Rc::new(ClassTypeData {
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
        Symbol(self.arena.allocate(SymbolKind::Type(TypeKind::EnumType(Rc::new(EnumTypeData {
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
        Symbol(self.arena.allocate(SymbolKind::Type(TypeKind::InterfaceType(Rc::new(InterfaceTypeData {
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
}