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
        //
    }
}