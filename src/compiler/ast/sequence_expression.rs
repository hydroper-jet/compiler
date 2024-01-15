use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct SequenceExpression {
    pub location: Location,
    pub left: Rc<Expression>,
    pub right: Rc<Expression>,
}