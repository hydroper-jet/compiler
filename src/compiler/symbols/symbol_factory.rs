use crate::ns::*;
use std::cell::Cell;

pub struct SymbolFactory<'a> {
    pub(crate) host: &'a SymbolHost,
    pub(crate) arena: &'a Arena<SymbolKind>,
}

impl<'a> SymbolFactory<'a> {
    pub fn create_unresolved(&self) -> UnresolvedSymbol {
        UnresolvedSymbol(Symbol(self.arena.allocate(SymbolKind::Unresolved(Cell::new(0)))))
    }
}