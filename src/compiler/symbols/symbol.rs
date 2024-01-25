use std::cell::Cell;
use std::ops::Deref;
use std::rc::Weak;

#[derive(Clone)]
pub struct Symbol(pub(crate) Weak<SymbolKind>);

impl Eq for Symbol {}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl Symbol {
    pub fn is_unresolved(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Unresolved(_))
    }
}

pub(crate) enum SymbolKind {
    Unresolved(Cell<u32>),
}

/// Unresolved symbol.
///
/// # Supported methods
/// 
/// * `unresolved_count()`
/// * `increment_unresolved_count()`
pub struct UnresolvedSymbol(pub Symbol);

impl Deref for UnresolvedSymbol {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_unresolved());
        &self.0
    }
}