use std::rc::Rc;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PlainMetadata {
    pub name: String,
    pub entries: Vec<Rc<PlainMetadataEntry>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlainMetadataEntry {
    pub key: Option<String>,
    pub value: Rc<PlainMetadataValue>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PlainMetadataValue {
    String(String),
    Number(f64),
    Boolean(bool),
    File {
        filename: String,
        data: Vec<u8>,
    },
    List(Vec<Rc<PlainMetadataEntry>>),
}