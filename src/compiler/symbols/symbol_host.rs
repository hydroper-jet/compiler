use crate::ns::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct SymbolHost {
    pub(crate) arena: Arena<SymbolKind>,
    pub(crate) unresolved: Symbol,
    pub(crate) any_type: Symbol,
    pub(crate) void_type: Symbol,
    pub(crate) function_types: HashMap<usize, Vec<Symbol>>,
    pub(crate) tuple_types: HashMap<usize, Vec<Symbol>>,
    pub(crate) nullable_types: HashMap<Symbol, Symbol>,
    pub(crate) types_after_explicit_type_substitution: HashMap<Symbol, Vec<Symbol>>,
    pub(crate) top_level_package: Symbol,
}

impl SymbolHost {
    pub fn new() -> Self {
        let arena = Arena::new();
        Self {
            arena: Arena::new(),
            unresolved: Symbol(arena.allocate(SymbolKind::Unresolved)),
            any_type: Symbol(arena.allocate(SymbolKind::Type(TypeKind::AnyType))),
            void_type: Symbol(arena.allocate(SymbolKind::Type(TypeKind::VoidType))),
            function_types: HashMap::new(),
            tuple_types: HashMap::new(),
            nullable_types: HashMap::new(),
            types_after_explicit_type_substitution: HashMap::new(),
            top_level_package: Symbol(arena.allocate(SymbolKind::Package(Rc::new(PackageData {
                name: String::new(),
                parent_definition: RefCell::new(None),
                properties: SharedMap::new(),
                redirect_packages: SharedArray::new(),
                subpackages: SharedMap::new(),
                jetdoc: RefCell::new(None),
            })))),
        }
    }

    pub fn factory(&mut self) -> SymbolFactory {
        SymbolFactory { host: self }
    }

    /// Returns the unique `Unresolved` symbol.
    pub fn unresolved(&self) -> Symbol {
        (self.unresolved).clone()
    }

    pub fn any_type(&self) -> Symbol {
        (self.any_type).clone()
    }

    pub fn void_type(&self) -> Symbol {
        (self.void_type).clone()
    }

    pub fn top_level_package(&self) -> Symbol {
        (self.top_level_package).clone()
    }
}