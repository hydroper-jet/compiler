use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub location: Location,
    pub directives: Vec<Rc<Directive>>,
}