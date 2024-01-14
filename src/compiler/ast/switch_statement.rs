use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct SwitchStatement {
    pub location: Location,
    pub discriminant: Rc<Expression>,
    pub cases: Vec<Case>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Case {
    pub labels: Vec<CaseLabel>,
    pub directives: Vec<Rc<Directive>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CaseLabel {
    Case(Rc<Expression>),
    Default,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SwitchTypeStatement {
    pub location: Location,
    pub discriminant: Rc<Expression>,
    pub cases: Vec<TypeCase>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TypeCase {
    /// Case binding. If `None`, designates a `default {}` case.
    pub binding: Option<Vec<TypedDestructuring>>,
    pub block: Rc<Block>,
}