use crate::ns::*;
use std::cell::Cell;

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
}