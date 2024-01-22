use std::rc::Rc;

#[derive(Clone)]
pub struct PlainMetadata {
    pub name: String,
    pub entries: Vec<Rc<PlainMetadataEntry>>,
}

#[derive(Clone)]
pub struct PlainMetadataEntry {
    pub key: Option<String>,
    pub value: Rc<PlainMetadataValue>,
}

#[derive(Clone)]
pub enum PlainMetadataValue {
    String(String),
    Number(f64),
    Boolean(bool),
    File {
        filename: String,
        data: Vec<u8>,
    },
    Collection(Vec<Rc<PlainMetadataEntry>>),
}