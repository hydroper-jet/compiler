use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct BreakStatement {
    pub location: Location,
    pub label: Option<(String, Location)>,
}