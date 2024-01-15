use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub location: Location,
    pub label: Option<Rc<Expression>>,
}