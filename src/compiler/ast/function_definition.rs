use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::{rc::Rc, str::FromStr};

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub location: Location,
    pub jetdoc: Option<Rc<JetDoc>>,
    pub attributes: Vec<Attribute>,
    pub name: FunctionName,
    pub type_parameters: Option<Vec<Rc<TypeParameter>>>,
    pub common: Rc<FunctionCommon>,
}

impl FunctionDefinition {
    /// Indicates whether the function definition is not a getter, setter,
    /// constructor, or proxy.
    pub fn is_normal(&self) -> bool {
        matches!(self.name, FunctionName::Identifier(_))
    }
    pub fn is_getter(&self) -> bool {
        matches!(self.name, FunctionName::Getter(_))
    }
    pub fn is_setter(&self) -> bool {
        matches!(self.name, FunctionName::Setter(_))
    }
    pub fn is_constructor(&self) -> bool {
        matches!(self.name, FunctionName::Constructor(_))
    }
    pub fn is_proxy(&self) -> bool {
        matches!(self.name, FunctionName::Proxy(_, _))
    }
    pub fn name_identifier(&self) -> (String, Location) {
        match &self.name {
            FunctionName::Identifier(name) => name.clone(),
            FunctionName::Getter(name) => name.clone(),
            FunctionName::Setter(name) => name.clone(),
            FunctionName::Constructor(name) => name.clone(),
            FunctionName::Proxy(_, name) => name.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FunctionName {
    Identifier((String, Location)),
    Getter((String, Location)),
    Setter((String, Location)),
    /// A `FunctionName` is a `Constructor` variant
    /// when the corresponding function definition is a constructor.
    Constructor((String, Location)),
    Proxy(ProxyKind, (String, Location)),
}

impl FunctionName {
    pub fn location(&self) -> Location {
        match self {
            Self::Identifier((_, l)) => l.clone(),
            Self::Getter((_, l)) => l.clone(),
            Self::Setter((_, l)) => l.clone(),
            Self::Constructor((_, l)) => l.clone(),
            Self::Proxy(_, (_, l)) => l.clone(),
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProxyKind {
    Positive,
    Negate,
    BitwiseNot,
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Power,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    ShiftLeft,
    ShiftRight,
    ShiftRightUnsigned,
    To,
    GetProperty,
    SetProperty,
    DeleteProperty,
    Has,
    Keys,
    Values,
}

impl TryFrom<ProxyKind> for Operator {
    type Error = ();
    fn try_from(value: ProxyKind) -> Result<Self, Self::Error> {
        match value {
            ProxyKind::Positive => Ok(Operator::Positive),
            ProxyKind::Negate => Ok(Operator::Negative),
            ProxyKind::BitwiseNot => Ok(Operator::BitwiseNot),
            ProxyKind::Add => Ok(Operator::Add),
            ProxyKind::Subtract => Ok(Operator::Subtract),
            ProxyKind::Multiply => Ok(Operator::Multiply),
            ProxyKind::Divide => Ok(Operator::Divide),
            ProxyKind::Remainder => Ok(Operator::Remainder),
            ProxyKind::Power => Ok(Operator::Power),
            ProxyKind::BitwiseAnd => Ok(Operator::BitwiseAnd),
            ProxyKind::BitwiseXor => Ok(Operator::BitwiseXor),
            ProxyKind::BitwiseOr => Ok(Operator::BitwiseOr),
            ProxyKind::ShiftLeft => Ok(Operator::ShiftLeft),
            ProxyKind::ShiftRight => Ok(Operator::ShiftRight),
            ProxyKind::ShiftRightUnsigned => Ok(Operator::ShiftRightUnsigned),
            ProxyKind::Has => Ok(Operator::In),
            _ => Err(()),
        }
    }
}

impl FromStr for ProxyKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "positive" => Ok(Self::Positive),
            "negate" => Ok(Self::Negate),
            "bitwiseNot" => Ok(Self::BitwiseNot),
            "add" => Ok(Self::Add),
            "subtract" => Ok(Self::Subtract),
            "multiply" => Ok(Self::Multiply),
            "divide" => Ok(Self::Divide),
            "remainder" => Ok(Self::Remainder),
            "power" => Ok(Self::Power),
            "bitwiseAnd" => Ok(Self::BitwiseAnd),
            "bitwiseXor" => Ok(Self::BitwiseXor),
            "bitwiseOr" => Ok(Self::BitwiseOr),
            "shiftLeft" => Ok(Self::ShiftLeft),
            "shiftRight" => Ok(Self::ShiftRight),
            "shiftRightUnsigned" => Ok(Self::ShiftRightUnsigned),
            "to" => Ok(Self::To),
            "getProperty" => Ok(Self::GetProperty),
            "setProperty" => Ok(Self::SetProperty),
            "deleteProperty" => Ok(Self::DeleteProperty),
            "has" => Ok(Self::Has),
            "keys" => Ok(Self::Keys),
            "values" => Ok(Self::Values),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionCommon {
    pub location: Location,
    /// Indicates whether the corresponding function
    /// contains the `yield` operator.
    pub contains_yield: bool,
    /// Indicates whether the corresponding function
    /// contains the `await` operator.
    pub contains_await: bool,
    pub signature: FunctionSignature,
    pub body: Option<FunctionBody>,
}

impl FunctionCommon {
    pub(crate) fn has_block_body(&self) -> bool {
        if let Some(ref body) = self.body { matches!(body, FunctionBody::Block(_)) } else { false }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub location: Location,
    pub parameters: Vec<Rc<Parameter>>,
    pub result_type: Option<Rc<Expression>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub location: Location,
    pub kind: ParameterKind,
    pub destructuring: TypedDestructuring,
    pub default_value: Option<Rc<Expression>>,
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u32)]
pub enum ParameterKind {
    Required = 1,
    Optional = 2,
    Rest = 3,
}

impl ParameterKind {
    pub fn may_be_followed_by(&self, other: Self) -> bool {
        (*self as u32) <= (other as u32)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FunctionBody {
    Expression(Rc<Expression>),
    Block(Rc<Block>),
}