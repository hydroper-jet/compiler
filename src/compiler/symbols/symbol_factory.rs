use crate::ns::*;

pub struct SymbolFactory<'a> {
    host: &'a SymbolHost,
    arena: &'a Arena<SymbolKind>,
}