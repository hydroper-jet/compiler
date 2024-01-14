use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionExpression {
    pub location: Location,
    pub name: Option<(String, Location)>,
    pub common: Rc<FunctionCommon>,
}