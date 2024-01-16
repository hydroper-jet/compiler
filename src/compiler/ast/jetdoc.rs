use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct JetDoc {
    pub location: Location,
    pub main_body: (String, Location),
    pub tags: Vec<JetDocTag>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum JetDocTag {
    Default(String),
    Event {
        name: String,
        description: String,
    },
    EventConstant(Rc<Expression>),
    EventType(Rc<Expression>),
    Example(String),
    Internal(String),
    Param {
        name: String,
        description: String,
    },
    Private,
    Return(String),
    See {
        reference: String,
        display_text: Option<String>,
    },
    Throws {
        class_reference: Rc<Expression>,
        description: Option<String>,
    },
}