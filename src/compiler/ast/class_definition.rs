use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ClassDefinition {
    pub location: Location,
    pub jetdoc: Option<Rc<JetDoc>>,
    pub attributes: Vec<Attribute>,
    pub allow_literal: bool,
    pub name: (String, Location),
    pub type_parameters: Option<Vec<Rc<TypeParameter>>>,
    pub extends_clause: Option<Rc<Expression>>,
    pub implements_clause: Option<Vec<Rc<Expression>>>,
    pub block: Rc<Block>,
}