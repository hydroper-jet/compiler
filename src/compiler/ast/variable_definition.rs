use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct VariableDefinition {
    pub location: Location,
    pub attributes: Vec<Attribute>,
    pub bindings: Vec<Rc<VariableBinding>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum VariableDefinitionKind {
    Var,
    Const,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VariableBinding {
    pub destructuring: TypedDestructuring,
    pub initializer: Option<Rc<Expression>>,
}