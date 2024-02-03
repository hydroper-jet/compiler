use crate::ns::*;
use std::rc::Rc;

/// Type substitution.
///
/// In the present, the Compiler codebase is able to perform type substitution on a limited
/// set of symbols and panics for unsupported symbols:
///
/// * Types
///   * Classes and interfaces are not substituted by `TypeSubstitution`, but may be in a future version.
/// * `Unresolved` is returned as is
/// * Variable properties
/// * Virtual properties
/// * Function symbols
/// 
/// In the future, type substitution may expand to other symbols if necessary.
pub struct TypeSubstitution<'a>(pub &'a SymbolHost);

impl<'a> TypeSubstitution<'a> {
    pub fn execute(&mut self, symbol: &Symbol, type_parameters: &SharedArray<Symbol>, substitute_types: &SharedArray<Symbol>) -> Symbol {
        if symbol.is_unresolved() {
            return symbol.clone();
        } else if symbol.is_type() {
            if symbol.is_function_type() {
                let result_type = symbol.result_type().type_substitution(self.0, type_parameters, substitute_types);
                let mut parameters: Vec<Rc<ParameterOfFunctionType>> = Vec::new();
                for param in symbol.parameters().iter() {
                    parameters.push(Rc::new(param.type_substitution(self.0, type_parameters, substitute_types)));
                }
                return self.0.factory().create_function_type(parameters, result_type);
            } else if symbol.is_nullable_type() {
                let base = &symbol.base().type_substitution(self.0, type_parameters, substitute_types);
                return self.0.factory().create_nullable_type(base);
            } else if symbol.is_tuple_type() {
                let el: Vec<Symbol> = symbol.element_types().iter().map(|t| t.type_substitution(self.0, type_parameters, substitute_types)).collect();
                return self.0.factory().create_tuple_type(el);
            } else if symbol.is_type_after_explicit_type_substitution() {
                let new_substitute_types: SharedArray<Symbol> = symbol.substitute_types().iter().map(|t| t.type_substitution(self.0, type_parameters, substitute_types)).collect();
                return self.0.factory().create_type_after_explicit_type_substitution(&symbol.origin(), &new_substitute_types);
            } else if symbol.is_type_parameter_type() {
                let i = type_parameters.index_of(&symbol);
                if let Some(i) = i {
                    return substitute_types.get(i).unwrap();
                }
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