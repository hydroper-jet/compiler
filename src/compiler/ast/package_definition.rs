use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct PackageDefinition {
    pub location: Location,
    pub name: Vec<(String, Location)>,
    pub block: Rc<Block>,
}