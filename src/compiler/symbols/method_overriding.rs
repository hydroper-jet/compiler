use crate::ns::*;

pub struct MethodOverriding<'a>(pub &'a mut SymbolHost);

impl<'a> MethodOverriding<'a> {
    pub fn abstract_methods_not_overriden(&mut self, class: &Symbol) -> Result<Vec<Symbol>, DeferVerificationError> {
        
    }
}