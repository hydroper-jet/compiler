use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ImportDirective {
    pub location: Location,
    pub alias: Option<(String, Location)>,
    pub package_name: Vec<(String, Location)>,
    pub import_specifier: ImportSpecifier,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ImportSpecifier {
    Wildcard,
    Identifier((String, Location)),
}