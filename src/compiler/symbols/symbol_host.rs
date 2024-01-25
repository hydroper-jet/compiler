use crate::ns::*;

pub struct SymbolHost {
    pub(crate) arena: Arena<SymbolKind>,
}

impl SymbolHost {
    pub fn factory(&self) -> SymbolFactory {
        SymbolFactory {
            host: self,
            arena: &self.arena,
        }
    }
}