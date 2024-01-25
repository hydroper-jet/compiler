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

    pub fn unresolved_count(&self) -> u32 {
        let symbol = self.0.upgrade().unwrap();
        let SymbolKind::Unresolved(ref symbol) = symbol.as_ref() else {
            panic!();
        };
        symbol.get()
    }

    pub fn increment_unresolved_count(&self) {
        let symbol = self.0.upgrade().unwrap();
        let SymbolKind::Unresolved(ref symbol) = symbol.as_ref() else {
            panic!();
        };
        symbol.set(symbol.get() + 1);
    }
}

pub(crate) enum SymbolKind {
    Unresolved(Cell<u32>),
}

/// Unresolved symbol.
///
/// # Supported methods
/// 
/// * `unresolved_count()` — Counter counting from zero (0).
/// * `increment_unresolved_count()`
pub struct UnresolvedSymbol(pub Symbol);

impl Deref for UnresolvedSymbol {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_unresolved());
        &self.0
    }
}