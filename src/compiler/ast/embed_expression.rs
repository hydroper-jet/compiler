use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct EmbedExpression {
    pub location: Location,
    pub source: String,
    pub from_clause: Option<(String, Location)>,
    pub as_clause: Option<Rc<Expression>>,
}