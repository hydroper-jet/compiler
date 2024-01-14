use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

/// Internal expression `()` used for arrow functions with an empty parameter list.
#[derive(Clone, Serialize, Deserialize)]
pub struct ArrowEmptyParameters {
    pub location: Location,
}