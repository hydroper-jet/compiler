use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ArrayLiteral {
    pub location: Location,
    pub elements: Vec<Element>,
    pub type_annotation: Option<Rc<Expression>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Element {
    Elision,
    Expression(Rc<Expression>),
    Rest((Rc<Expression>, Location)),
}