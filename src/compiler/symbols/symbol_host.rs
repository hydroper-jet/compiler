use crate::ns::*;

pub struct SymbolHost {
    pub(crate) arena: Arena<SymbolKind>,
}