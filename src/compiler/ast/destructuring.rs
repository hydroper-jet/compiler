use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct TypedDestructuring {
    pub location: Location,
    pub destructuring: Rc<Expression>,
    pub non_null: bool,
    pub type_annotation: Option<Rc<Expression>>,
}