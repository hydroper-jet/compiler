use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct ArrowFunctionExpression {
    pub location: Location,
    pub common: Rc<FunctionCommon>,
}