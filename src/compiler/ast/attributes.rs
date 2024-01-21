use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::{rc::Rc, str::FromStr};

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

    pub fn has(list: &Vec<Attribute>, attribute: &Attribute) -> bool {
        match attribute {
            Self::Public(_) => Self::find_public(list).is_some(),
            Self::Private(_) => Self::find_private(list).is_some(),
            Self::Protected(_) => Self::find_protected(list).is_some(),
            Self::Internal(_) => Self::find_internal(list).is_some(),
            Self::Proxy(_) => Self::find_proxy(list).is_some(),
            Self::Final(_) => Self::find_final(list).is_some(),
            Self::Native(_) => Self::find_native(list).is_some(),
            Self::Static(_) => Self::find_static(list).is_some(),
            Self::Abstract(_) => Self::find_abstract(list).is_some(),
            Self::Override(_) => Self::find_override(list).is_some(),
            _ => false,
        }
    }

    pub fn is_duplicate_visibility(list: &Vec<Attribute>, attribute: &Attribute) -> bool {
        match attribute {
            Self::Public(_) |
            Self::Private(_) |
            Self::Protected(_) |
            Self::Internal(_) => Self::find_public(list).is_some() || Self::find_private(list).is_some() || Self::find_protected(list).is_some() || Self::find_internal(list).is_some(),
            _ => false,
        }
    }

    pub fn is_metadata(&self) -> bool { matches!(self, Self::Metadata(_)) }
    pub fn is_public(&self) -> bool { matches!(self, Self::Public(_)) }
    pub fn is_private(&self) -> bool { matches!(self, Self::Private(_)) }
    pub fn is_protected(&self) -> bool { matches!(self, Self::Protected(_)) }
    pub fn is_internal(&self) -> bool { matches!(self, Self::Internal(_)) }
    pub fn is_proxy(&self) -> bool { matches!(self, Self::Proxy(_)) }
    pub fn is_final(&self) -> bool { matches!(self, Self::Final(_)) }
    pub fn is_native(&self) -> bool { matches!(self, Self::Native(_)) }
    pub fn is_static(&self) -> bool { matches!(self, Self::Static(_)) }
    pub fn is_abstract(&self) -> bool { matches!(self, Self::Abstract(_)) }
    pub fn is_override(&self) -> bool { matches!(self, Self::Override(_)) }

    pub fn from_identifier_name(name: &str, location: &Location) -> Option<Attribute> {
        if location.character_count() != name.chars().count() {
            return None;
        }
        match name.as_ref() {
            "proxy" => Some(Attribute::Proxy(location.clone())),
            "final" => Some(Attribute::Final(location.clone())),
            "native" => Some(Attribute::Native(location.clone())),
            "static" => Some(Attribute::Static(location.clone())),
            "abstract" => Some(Attribute::Abstract(location.clone())),
            "override" => Some(Attribute::Override(location.clone())),
            _ => None,
        }
    }
}