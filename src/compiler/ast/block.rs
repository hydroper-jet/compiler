use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub location: Location,
    /// Block meta-data for block statements.
    pub metadata: Option<Vec<Attribute>>,
    pub directives: Vec<Rc<Directive>>,
}