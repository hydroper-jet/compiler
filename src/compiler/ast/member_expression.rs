use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct MemberExpression {
    pub location: Location,
    pub base: Rc<Expression>,
    pub identifier: QualifiedIdentifier,
}