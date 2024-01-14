use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct WhileStatement {
    pub location: Location,
    pub test: Rc<Expression>,
    pub body: Rc<Directive>,
}