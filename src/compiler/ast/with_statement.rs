use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct WithStatement {
    pub location: Location,
    pub object: Rc<Expression>,
    pub body: Rc<Directive>,
}