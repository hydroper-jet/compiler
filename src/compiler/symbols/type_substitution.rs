use crate::ns::*;

pub struct TypeSubstitution<'a>(pub &'a SymbolHost);

impl<'a> TypeSubstitution<'a> {
    pub fn execute(&mut self, symbol: &Symbol, type_parameters: &SharedArray<Symbol>, substitute_types: &SharedArray<Symbol>) -> Symbol {
        //
    }
}