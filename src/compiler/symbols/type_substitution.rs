use crate::ns::*;

pub struct TypeSubstitution<'a>(pub &'a mut SymbolHost);

impl<'a> TypeSubstitution<'a> {
    pub fn execute(&mut self, symbol: &Symbol, type_parameters: &SharedArray<Symbol>, substitute_types: &SharedArray<Symbol>) -> Symbol {
        // * Handle types.
        // * Handle variable properties, virtual properties and functions.
        //   * Functions result into `FunctionAfterExplicitOrIndirectTypeSubstitution`.
        // * Handle `Unresolved`... as well.
        unimplemented!();
    }
}