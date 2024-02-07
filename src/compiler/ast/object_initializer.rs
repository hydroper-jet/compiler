use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ObjectInitializer {
    pub location: Location,
    pub fields: Vec<Rc<InitializerField>>,
    pub type_annotation: Option<Rc<Expression>>,
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
                value.clone().map_or(name.1.clone(), |v| name.1.combine_with(v.location()))
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

impl FieldName {
    pub(crate) fn id(&self) -> Option<String> {
        let Self::Identifier(id) = &self else {
            return None;
        };
        Some(id.clone())
    }

    pub(crate) fn id_equals(&self, name: &str) -> bool {
        self.id().map(|name1| name == name1).unwrap_or(false)
    }
}