use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub location: Location,
    pub kind: AttributeKind,
}

impl Attribute {
    pub fn find_metadata(list: &Vec<Attribute>) -> Vec<Rc<Expression>> {
        let mut r = vec![];
        for a in list {
            match &a.kind {
                AttributeKind::Metadata(e) => {
                    r.push(e.clone());
                },
                _ => {},
            }
        }
        r
    }
    pub fn find_public(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a.kind { AttributeKind::Public(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_private(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a.kind { AttributeKind::Private(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_protected(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a.kind { AttributeKind::Protected(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_internal(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a.kind { AttributeKind::Internal(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_proxy(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a.kind { AttributeKind::Proxy(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_final(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a.kind { AttributeKind::Final(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_native(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a.kind { AttributeKind::Native(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_static(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a.kind { AttributeKind::Static(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_abstract(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a.kind { AttributeKind::Abstract(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_override(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a.kind { AttributeKind::Override(l) => return Some(l.clone()), _ => return None } }; None }
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