use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigurationDirective {
    pub location: Location,
    pub directive: Rc<Directive>,
}