use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct VariableDefinition {
    pub location: Location,
    pub jetdoc: Option<Rc<JetDoc>>,
    pub attributes: Vec<Attribute>,
    pub kind: (VariableDefinitionKind, Location),
    pub bindings: Vec<Rc<VariableBinding>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum VariableDefinitionKind {
    Var,
    Const,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SimpleVariableDefinition {
    pub location: Location,
    pub kind: (VariableDefinitionKind, Location),
    pub bindings: Vec<Rc<VariableBinding>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VariableBinding {
    pub destructuring: TypedDestructuring,
    pub initializer: Option<Rc<Expression>>,
}

impl VariableBinding {
    pub fn location(&self) -> Location {
        self.initializer.map_or(self.destructuring.location.clone(), |init| self.destructuring.location.combine_with(init.location()))
    }
}