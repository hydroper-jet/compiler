use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct TryStatement {
    pub location: Location,
    pub block: Rc<Block>,
    pub catch_clauses: Vec<CatchClause>,
    pub finally_clause: Option<FinallyClause>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CatchClause {
    pub location: Location,
    pub destructuring: Rc<TypedDestructuring>,
    pub block: Rc<Block>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FinallyClause {
    pub location: Location,
    pub block: Rc<Block>,
}