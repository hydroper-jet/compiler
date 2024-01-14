use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

/// Directive attached with a source location.
#[derive(Clone, Serialize, Deserialize)]
pub enum Directive {
    EmptyStatement(EmptyStatement),
    ExpressionStatement(ExpressionStatement),
    SuperStatement(SuperStatement),
    Block(Block),
    LabeledStatement(LabeledStatement),
    IfStatement(IfStatement),
    SwitchStatement(SwitchStatement),
    SwitchTypeStatement(SwitchTypeStatement),
    DoStatement(DoStatement),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
    ForInStatement(ForInStatement),
}

impl Directive {
    pub fn location(&self) -> Location {
        match self {
            Self::EmptyStatement(d) => d.location.clone(),
            Self::ExpressionStatement(d) => d.location.clone(),
            Self::SuperStatement(d) => d.location.clone(),
            Self::Block(d) => d.location.clone(),
            Self::LabeledStatement(d) => d.location.clone(),
            Self::IfStatement(d) => d.location.clone(),
            Self::SwitchStatement(d) => d.location.clone(),
            Self::SwitchTypeStatement(d) => d.location.clone(),
            Self::DoStatement(d) => d.location.clone(),
            Self::WhileStatement(d) => d.location.clone(),
            Self::ForStatement(d) => d.location.clone(),
            Self::ForInStatement(d) => d.location.clone(),
        }
    }
}