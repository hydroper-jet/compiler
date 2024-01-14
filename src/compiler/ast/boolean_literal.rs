use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct BooleanLiteral {
    pub location: Location,
    pub value: bool,
}