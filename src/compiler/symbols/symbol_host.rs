use crate::ns::*;
use std::collections::HashMap;

pub struct SymbolHost {
    pub(crate) arena: Arena<SymbolKind>,
    pub(crate) any_type: Symbol,
    pub(crate) void_type: Symbol,
    pub(crate) function_types: HashMap<usize, Vec<Symbol>>,
    pub(crate) tuple_types: HashMap<usize, Vec<Symbol>>,
    pub(crate) nullable_types: HashMap<Symbol, Symbol>,
}

impl SymbolHost {
    pub fn new() -> Self {
        let arena = Arena::new();
        Self {
            arena: Arena::new(),
            any_type: Symbol(arena.allocate(SymbolKind::Type(TypeKind::AnyType))),
            void_type: Symbol(arena.allocate(SymbolKind::Type(TypeKind::VoidType))),
            function_types: HashMap::new(),
            tuple_types: HashMap::new(),
            nullable_types: HashMap::new(),
        }
    }

    pub fn factory(&mut self) -> SymbolFactory {
        SymbolFactory { host: self }
    }

    pub fn any_type(&self) -> Symbol {
        self.any_type.clone()
    }

    pub fn void_type(&self) -> Symbol {
        self.void_type.clone()
    }
}