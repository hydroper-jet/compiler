use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct LabeledStatement {
    pub location: Location,
    pub label: (String, Location),
    pub substatement: Rc<Directive>,
}