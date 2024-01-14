use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct OptionalTypeExpression {
    pub location: Location,
    pub base: Rc<Expression>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AnyTypeExpression {
    pub location: Location,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VoidTypeExpression {
    pub location: Location,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UndefinedTypeExpression {
    pub location: Location,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ArrayTypeExpression {
    pub location: Location,
    pub expression: Rc<Expression>,
}

/// A tuple type expression consisting of at least two elements.
#[derive(Clone, Serialize, Deserialize)]
pub struct TupleTypeExpression {
    pub location: Location,
    pub expressions: Vec<Rc<Expression>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionTypeExpression {
    pub location: Location,
    pub parameters: Vec<FunctionTypeParameter>,
    pub result_type: Rc<Expression>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionTypeParameter {
    pub location: Location,
    pub kind: FunctionParameterKind,
    pub name: (String, Location),
    pub type_annotation: Option<Rc<Expression>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FunctionParameterKind {
    Required,
    Optional,
    Rest,
}