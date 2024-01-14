use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

/// The `import.meta` expression.
#[derive(Clone, Serialize, Deserialize)]
pub struct ImportMeta {
    pub location: Location,
}