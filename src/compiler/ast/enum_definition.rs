use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct EnumDefinition {
    pub location: Location,
    pub jetdoc: Option<Rc<JetDoc>>,
    pub attributes: Vec<Attribute>,
    pub name: (String, Location),
    pub as_clause: Option<Rc<Expression>>,
    pub block: Rc<Block>,
}