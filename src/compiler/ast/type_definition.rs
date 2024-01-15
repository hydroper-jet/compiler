use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    pub location: Location,
    pub attributes: Vec<Attribute>,
    pub left: (String, Location),
    pub right: Option<Rc<Expression>>,
}