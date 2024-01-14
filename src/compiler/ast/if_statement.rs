use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct IfStatement {
    pub location: Location,
    pub test: Rc<Expression>,
    pub consequent: Rc<Directive>,
    pub alternative: Option<Rc<Directive>>,
}