use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

/// The `o.<...>` expression.
#[derive(Clone, Serialize, Deserialize)]
pub struct ExpressionWithTypeArguments {
    pub location: Location,
    pub base: Rc<Expression>,
    pub arguments: Vec<Rc<Expression>>,
}