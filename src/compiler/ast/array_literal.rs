use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ArrayLiteral {
    pub location: Location,
    pub elements: Vec<Option<Element>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Element {
    Expression(Rc<Expression>),
    Rest((Rc<Expression>, Location)),
}