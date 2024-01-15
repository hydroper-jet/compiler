use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct UseDirective {
    pub location: Location,
    pub alias: Option<(String, Location)>,
    pub package_name: Vec<(String, Location)>,
    pub import_specifier: ImportSpecifier,
}