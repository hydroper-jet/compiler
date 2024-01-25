use crate::ns::*;

pub struct SymbolHost {
    pub(crate) arena: Arena<SymbolKind>,
    pub(crate) any_type: Symbol,
    pub(crate) void_type: Symbol,
}

impl SymbolHost {
    pub fn new(&self) -> Self {
        Self {
            arena: Arena::new(),
            any_type: Symbol(self.arena.allocate(SymbolKind::Type(TypeKind::AnyType))),
            void_type: Symbol(self.arena.allocate(SymbolKind::Type(TypeKind::VoidType))),
        }
    }

    pub fn factory(&self) -> SymbolFactory {
        SymbolFactory {
            host: self,
            arena: &self.arena,
        }
    }

    pub fn any_type(&self) -> Symbol {
        self.any_type.clone()
    }

    pub fn void_type(&self) -> Symbol {
        self.void_type.clone()
    }
}