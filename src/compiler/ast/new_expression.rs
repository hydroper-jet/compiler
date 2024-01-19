use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct NewExpression {
    pub location: Location,
    pub base: Rc<Expression>,
    pub arguments: Option<Vec<Rc<Expression>>>,
}