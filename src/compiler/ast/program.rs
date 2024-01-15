use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct Program {
    pub location: Location,
    pub packages: Vec<Rc<PackageDefinition>>,
    pub directives: Vec<Rc<Directive>>,
}