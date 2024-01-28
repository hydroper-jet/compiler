use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct JetDoc {
    pub location: Location,
    pub main_body: Option<(String, Location)>,
    pub tags: Vec<(JetDocTag, Location)>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum JetDocTag {
    Default(String),
    Deprecated {
        message: Option<String>,
    },
    Event {
        name: String,
        description: String,
    },
    EventType(Rc<Expression>),
    Example(String),
    Image {
        path: String,
    },
    Internal(String),
    Param {
        name: String,
        description: String,
    },
    Private,
    Return(String),
    See {
        reference: Rc<JetDocReference>,
        display_text: Option<String>,
    },
    Throws {
        class_reference: Rc<Expression>,
        description: Option<String>,
    },
}

/// A JetDoc reference consisting of an optional base and
/// an optional instance property fragment (`#x`).
#[derive(Clone, Serialize, Deserialize)]
pub struct JetDocReference {
    /// Base expression.
    pub base: Option<Rc<Expression>>,
    /// Instance property fragment following the hash character.
    pub instance_property: Option<String>,
}