use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub location: Location,
    pub kind: AttributeKind,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AttributeKind {
    Metadata(Rc<Expression>),
    Public(Location),
    Private(Location),
    Protected(Location),
    Internal(Location),
    Proxy(Location),
    Final(Location),
    Native(Location),
    Static(Location),
    Abstract(Location),
    Override(Location),
}

impl AttributeKind {
    pub fn location(&self) -> Location {
        match self {
            Self::Metadata(a) => a.location(),
            Self::Public(a) => a.clone(),
            Self::Private(a) => a.clone(),
            Self::Protected(a) => a.clone(),
            Self::Internal(a) => a.clone(),
            Self::Proxy(a) => a.clone(),
            Self::Final(a) => a.clone(),
            Self::Native(a) => a.clone(),
            Self::Static(a) => a.clone(),
            Self::Abstract(a) => a.clone(),
            Self::Override(a) => a.clone(),
        }
    }
}