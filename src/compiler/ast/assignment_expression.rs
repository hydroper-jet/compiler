use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct AssignmentExpression {
    pub location: Location,
    pub compound: Operator,
    /// Assignment left-hand side.
    /// 
    /// If the left-hand side is an `ObjectInitializer` or an `ArrayLiteral`,
    /// it is a destructuring pattern.
    pub left: Rc<Expression>,
    pub right: Rc<Expression>,
}