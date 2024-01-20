use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub enum Attribute {
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

impl Attribute {
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

    pub fn find_metadata(list: &Vec<Attribute>) -> Vec<Rc<Expression>> {
        let mut r = vec![];
        for a in list {
            match &a {
                Self::Metadata(e) => {
                    r.push(e.clone());
                },
                _ => {},
            }
        }
        r
    }
    pub fn find_public(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Public(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_private(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Private(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_protected(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Protected(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_internal(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Internal(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_proxy(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Proxy(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_final(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Final(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_native(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Native(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_static(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Static(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_abstract(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Abstract(l) => return Some(l.clone()), _ => return None } }; None }
    pub fn find_override(list: &Vec<Attribute>) -> Option<Location> { for a in list { match &a { Self::Override(l) => return Some(l.clone()), _ => return None } }; None }
}