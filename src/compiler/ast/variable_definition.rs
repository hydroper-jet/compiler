use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

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