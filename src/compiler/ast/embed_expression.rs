use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct EmbedExpression {
    pub location: Location,
    pub description: ObjectInitializer,
}