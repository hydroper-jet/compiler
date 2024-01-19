use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub location: Location,
    pub jetdoc: Option<Rc<JetDoc>>,
    pub attributes: Vec<Attribute>,
    pub name: FunctionName,
    pub type_parameters: Vec<Rc<TypeParameter>>,
    pub common: Rc<FunctionCommon>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FunctionName {
    Identifier((String, Location)),
    Getter((String, Location)),
    Setter((String, Location)),
    /// A `FunctionName` is a `Constructor` variant
    /// when the corresponding function definition is a constructor.
    Constructor((String, Location)),
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