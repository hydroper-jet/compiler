use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct DescendantsExpression {
    pub location: Location,
    pub base: Rc<Expression>,
    pub identifier: QualifiedIdentifier,
}