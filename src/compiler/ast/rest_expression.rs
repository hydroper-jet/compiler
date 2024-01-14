use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

/// Internal `...x` expression used for array literals and
/// arrow function signatures.
#[derive(Clone, Serialize, Deserialize)]
pub struct RestExpression {
    pub location: Location,
    pub expression: Rc<Expression>,
}