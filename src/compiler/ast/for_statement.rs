use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ForStatement {
    pub location: Location,
    pub init: Option<Rc<ForInitializer>>,
    pub test: Option<Rc<Expression>>,
    pub update: Option<Rc<Expression>>,
    pub body: Rc<Directive>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ForInitializer {
    Expression(Rc<Expression>),
    VariableDefinition {
        kind: VariableDefinitionKind,
        bindings: Vec<Rc<VariableBinding>>,
    },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ForInStatement {
    pub location: Location,
    pub each: bool,
    pub left: Rc<ForInBinding>,
    pub right: Rc<Expression>,
    pub body: Rc<Directive>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ForInBinding {
    Expression(Rc<Expression>),
    VariableDefinition {
        kind: VariableDefinitionKind,
        binding: Rc<VariableBinding>,
    },
}