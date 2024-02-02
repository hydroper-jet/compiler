use crate::ns::*;

pub struct MethodOverriding<'a>(pub &'a mut SymbolHost);

impl<'a> MethodOverriding<'a> {
    pub fn abstract_methods_not_overriden(&mut self, class: &Symbol) -> Result<Vec<Symbol>, DeferVerificationError> {
        let base_class = class.extends_class(self.0);
        if base_class.is_none() {
            return Ok(vec![]);
        }
        let base_class = base_class.unwrap();
        if base_class.is_unresolved() {
            return Err(DeferVerificationError);
        }
        let mut r: Vec<Symbol> = vec![];
        for (name, prop) in base_class.prototype(self.0).borrow().iter() {
            // Regular method
            if prop.is_function() {
                if prop.is_abstract() {
                    let prop2 = class.prototype(self.0).get(name);
                    if prop2.is_none() || !prop2.unwrap().is_function() {
                        r.push(prop.clone());
                    }
                }
            }
            // Accessor
            else if prop.is_virtual_property() {
                if let Some(getter) = prop.getter(self.0) {
                    if prop.not_overriden_abstract_getter(&getter, class, self.0) {
                        r.push(prop.clone());
                    }
                }
                if let Some(setter) = prop.setter(self.0) {
                    if prop.not_overriden_abstract_setter(&setter, class, self.0) {
                        r.push(prop.clone());
                    }
                }
            }
        }
        Ok(r)
    }
}