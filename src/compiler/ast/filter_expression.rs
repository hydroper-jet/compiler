use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

/// Filter operation `o.(condition)`.
#[derive(Clone, Serialize, Deserialize)]
pub struct FilterExpression {
    pub location: Location,
    pub base: Rc<Expression>,
    pub test: Rc<Expression>,
}