use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct InterfaceDefinition {
    pub location: Location,
    pub attributes: Vec<Attribute>,
    pub name: (String, Location),
    pub type_parameters: Vec<Rc<TypeParameter>>,
    pub extends_clause: Option<Vec<Rc<Expression>>>,
    pub block: Rc<Block>,
}