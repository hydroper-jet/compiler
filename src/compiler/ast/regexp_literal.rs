use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct RegExpLiteral {
    pub location: Location,
    pub body: String,
    pub flags: String,
}