use crate::ns::*;
use std::rc::Rc;

pub struct PropertyResolution<'a>(pub &'a mut SymbolHost);

#[derive(Clone)]
pub enum SemanticPropertyKey {
    String(String),
    Number(f64),
    Value(Symbol),
}

impl SemanticPropertyKey {
    pub fn symbol(&self, host: &mut SymbolHost) -> Symbol {
        match self {
            Self::String(s) => host.factory().create_string_constant(s.clone(), &host.string_type()),
            Self::Number(d) => host.factory().create_number_constant(AbstractRangeNumber::Number(d.clone()), &host.number_type()),
            Self::Value(s) => s.clone(),
        }
    }

    pub fn string_value(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.clone()),
            Self::Value(s) => {
                if s.is_string_constant() {
                    Some(s.string_value())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    pub fn number_value(&self) -> Option<f64> {
        match self {
            Self::Number(d) => Some(*d),
            Self::Value(d) => {
                if d.is_number_constant() {
                    match d.number_value() {
                        AbstractRangeNumber::Number(d) => Some(d),
                        _ => None,
                    }
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}

impl<'a> PropertyResolution<'a> {
    pub fn resolve_property(&mut self, base: &Symbol, qual: Option<Symbol>, key: SemanticPropertyKey) -> Result<Option<Symbol>, PropertyResolutionError> {
        // 1. If base is a value whose type is one of { XML, XMLList }, return XMLReferenceValue(base, qual, key).
        if base.is_value() && [self.0.xml_type(), self.0.xml_list_type()].contains(&base.static_type(self.0)) {
            let k = &key.symbol(self.0);
            return Ok(Some(self.0.factory().create_xml_reference_value(base, qual, k)));
        }
        ()
    }
}