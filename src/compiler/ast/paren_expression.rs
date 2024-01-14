use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ParenExpression {
    pub location: Location,
    pub expression: Rc<Expression>,
}