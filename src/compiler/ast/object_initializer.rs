use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ObjectInitializer {
    pub location: Location,
    pub fields: Vec<Rc<InitializerFieldOrRest>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum InitializerFieldOrRest {
    Expression(Rc<InitializerField>),
    Rest((Rc<Expression>, Location)),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InitializerField {
    pub location: Location,
    pub name: FieldName,
    /// Non-null operator used for destructuring.
    pub non_null: bool,
    pub value: Option<Rc<Expression>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FieldName {
    Identifier((String, Location)),
    Brackets(Rc<InitializerField>),
    StringLiteral(Rc<Expression>),
    NumericLiteral(Rc<Expression>),
}