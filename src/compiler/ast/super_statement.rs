use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct SuperStatement {
    pub location: Location,
    pub arguments: Vec<Rc<Expression>>,
}