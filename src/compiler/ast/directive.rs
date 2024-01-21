use crate::ns::*;
use serde::{Serialize, Deserialize};

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
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    WithStatement(WithStatement),
    ReturnStatement(ReturnStatement),
    ThrowStatement(ThrowStatement),
    DefaultXmlNamespaceStatement(DefaultXmlNamespaceStatement),
    TryStatement(TryStatement),
    ConfigurationDirective(ConfigurationDirective),
    ImportDirective(ImportDirective),
    UseDirective(UseDirective),
    VariableDefinition(VariableDefinition),
    FunctionDefinition(FunctionDefinition),
    ClassDefinition(ClassDefinition),
    EnumDefinition(EnumDefinition),
    InterfaceDefinition(InterfaceDefinition),
    TypeDefinition(TypeDefinition),
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
            Self::BreakStatement(d) => d.location.clone(),
            Self::ContinueStatement(d) => d.location.clone(),
            Self::WithStatement(d) => d.location.clone(),
            Self::ReturnStatement(d) => d.location.clone(),
            Self::ThrowStatement(d) => d.location.clone(),
            Self::DefaultXmlNamespaceStatement(d) => d.location.clone(),
            Self::TryStatement(d) => d.location.clone(),
            Self::ConfigurationDirective(d) => d.location.clone(),
            Self::ImportDirective(d) => d.location.clone(),
            Self::UseDirective(d) => d.location.clone(),
            Self::VariableDefinition(d) => d.location.clone(),
            Self::FunctionDefinition(d) => d.location.clone(),
            Self::ClassDefinition(d) => d.location.clone(),
            Self::EnumDefinition(d) => d.location.clone(),
            Self::InterfaceDefinition(d) => d.location.clone(),
            Self::TypeDefinition(d) => d.location.clone(),
        }
    }
}