use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct DefaultXmlNamespaceStatement {
    pub location: Location,
    pub right: Rc<Expression>,
}