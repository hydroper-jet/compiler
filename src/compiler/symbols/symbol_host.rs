use crate::ns::*;
use std::collections::HashMap;

pub struct SymbolHost {
    pub(crate) arena: Arena<SymbolKind>,
    pub(crate) any_type: Symbol,
    pub(crate) void_type: Symbol,
    pub(crate) function_types: HashMap<usize, Vec<Symbol>>,
}

impl SymbolHost {
    pub fn new(&self) -> Self {
        Self {
            arena: Arena::new(),
            any_type: Symbol(self.arena.allocate(SymbolKind::Type(TypeKind::AnyType))),
            void_type: Symbol(self.arena.allocate(SymbolKind::Type(TypeKind::VoidType))),
            function_types: HashMap::new(),
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