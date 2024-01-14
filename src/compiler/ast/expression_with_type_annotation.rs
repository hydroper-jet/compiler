use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

/// Internal expression `x: T` used for arrow function signatures.
#[derive(Clone, Serialize, Deserialize)]
pub struct ExpressionWithTypeAnnotation {
    pub location: Location,
    pub base: Rc<Expression>,
    pub type_annotation: Rc<Expression>,
}