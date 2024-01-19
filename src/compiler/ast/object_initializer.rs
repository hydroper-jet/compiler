use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ObjectInitializer {
    pub location: Location,
    pub fields: Vec<Rc<InitializerField>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum InitializerField {
    Field {
        name: (FieldName, Location),
        /// Non-null operator used for destructuring.
        non_null: bool,
        value: Option<Rc<Expression>>,
    },
    Rest((Rc<Expression>, Location)),
}

impl InitializerField {
    pub fn location(&self) -> Location {
        match self {
            Self::Field { ref name, ref value, .. } => {
                value.map_or(name.1.clone(), |v| name.1.combine_with(v.location()))
            },
            Self::Rest((_, ref l)) => l.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FieldName {
    Identifier(String),
    Brackets(Rc<Expression>),
    StringLiteral(Rc<Expression>),
    NumericLiteral(Rc<Expression>),
}