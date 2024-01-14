use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct QualifiedIdentifier {
    pub location: Location,
    pub attribute: bool,
    pub qualifier: Option<Rc<Expression>>,
    pub id: QualifiedIdentifierIdentifier,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum QualifiedIdentifierIdentifier {
    Id((String, Location)),
    Brackets(Rc<Expression>),
}