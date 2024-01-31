use crate::ns::*;
use std::rc::Rc;

pub struct TypeSubstitution<'a>(pub &'a mut SymbolHost);

impl<'a> TypeSubstitution<'a> {
    pub fn execute(&mut self, symbol: &Symbol, type_parameters: &SharedArray<Symbol>, substitute_types: &SharedArray<Symbol>) -> Symbol {
        // * Handle types.
        // * Handle variable properties, virtual properties and functions.
        //   * Functions result into `FunctionAfterExplicitOrIndirectTypeSubstitution`.
        // * Handle `Unresolved`... as well.
        if symbol.is_unresolved() {
            return symbol.clone();
        } else if symbol.is_type() {
            if symbol.is_function_type() {
                let result_type = self.execute(&symbol.result_type(), type_parameters, substitute_types);
                let mut parameters: Vec<Rc<ParameterOfFunctionType>> = Vec::new();
                for param in symbol.parameters().iter() {
                    parameters.push(Rc::new(param.type_substitution(self.0, type_parameters, substitute_types)));
                }
                return self.0.factory().create_function_type(parameters, result_type);
            }
            return symbol.clone();
        } else if symbol.is_variable_property() {
            self.0.factory().create_variable_property_after_indirect_type_substitution(symbol, type_parameters, substitute_types)
        } else if symbol.is_virtual_property() {
            self.0.factory().create_virtual_property_after_indirect_type_substitution(symbol, type_parameters, substitute_types)
        } else if symbol.is_function() {
            self.0.factory().create_function_after_explicit_or_indirect_type_substitution(symbol, type_parameters, substitute_types)
        } else {
            panic!()
        }
    }
}