use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

/// Expression attached with a source location.
#[derive(Clone, Serialize, Deserialize)]
pub enum Expression {
    QualifiedIdentifier(QualifiedIdentifier),
    Embed(EmbedExpression),
    Paren(ParenExpression),
    NullLiteral(NullLiteral),
    BooleanLiteral(BooleanLiteral),
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
    ThisLiteral(ThisLiteral),
    RegExpLiteral(RegExpLiteral),
    Xml(XmlExpression),
    XmlMarkup(XmlMarkupExpression),
    XmlList(XmlListExpression),
    ArrayLiteral(ArrayLiteral),
    ObjectInitializer(ObjectInitializer),
    Function(FunctionExpression),
    ImportMeta(ImportMeta),
    New(NewExpression),
    Member(MemberExpression),
    ComputedMember(ComputedMemberExpression),
    Descendants(DescendantsExpression),
    Filter(FilterExpression),
    Super(SuperExpression),
    Call(CallExpression),
    WithTypeArguments(ExpressionWithTypeArguments),
    Unary(UnaryExpression),
    OptionalChaining(OptionalChainingExpression),
    OptionalChainingPlaceholder(OptionalChainingPlaceholder),
    Binary(BinaryExpression),
    Conditional(ConditionalExpression),
    Assignment(AssignmentExpression),
    Sequence(SequenceExpression),
    NullableType(NullableTypeExpression),
    NonNullableType(NonNullableTypeExpression),
    AnyType(AnyTypeExpression),
    VoidType(VoidTypeExpression),
    ArrayType(ArrayTypeExpression),
    TupleType(TupleTypeExpression),
    FunctionType(FunctionTypeExpression),
}

impl Expression {
    pub fn location(&self) -> Location {
        match self {
            Self::QualifiedIdentifier(e) => e.location.clone(),
            Self::Embed(e) => e.location.clone(),
            Self::Paren(e) => e.location.clone(),
            Self::NullLiteral(e) => e.location.clone(),
            Self::BooleanLiteral(e) => e.location.clone(),
            Self::NumericLiteral(e) => e.location.clone(),
            Self::StringLiteral(e) => e.location.clone(),
            Self::ThisLiteral(e) => e.location.clone(),
            Self::RegExpLiteral(e) => e.location.clone(),
            Self::Xml(e) => e.location.clone(),
            Self::XmlMarkup(e) => e.location.clone(),
            Self::XmlList(e) => e.location.clone(),
            Self::ArrayLiteral(e) => e.location.clone(),
            Self::ObjectInitializer(e) => e.location.clone(),
            Self::Function(e) => e.location.clone(),
            Self::ImportMeta(e) => e.location.clone(),
            Self::New(e) => e.location.clone(),
            Self::Member(e) => e.location.clone(),
            Self::ComputedMember(e) => e.location.clone(),
            Self::Descendants(e) => e.location.clone(),
            Self::Filter(e) => e.location.clone(),
            Self::Super(e) => e.location.clone(),
            Self::Call(e) => e.location.clone(),
            Self::WithTypeArguments(e) => e.location.clone(),
            Self::Unary(e) => e.location.clone(),
            Self::OptionalChaining(e) => e.location.clone(),
            Self::OptionalChainingPlaceholder(e) => e.location.clone(),
            Self::Binary(e) => e.location.clone(),
            Self::Conditional(e) => e.location.clone(),
            Self::Assignment(e) => e.location.clone(),
            Self::Sequence(e) => e.location.clone(),
            Self::NullableType(e) => e.location.clone(),
            Self::NonNullableType(e) => e.location.clone(),
            Self::AnyType(e) => e.location.clone(),
            Self::VoidType(e) => e.location.clone(),
            Self::ArrayType(e) => e.location.clone(),
            Self::TupleType(e) => e.location.clone(),
            Self::FunctionType(e) => e.location.clone(),
        }
    }

    pub(crate) fn to_metadata(&self) -> Option<Vec<Attribute>> {
        match self {
            Self::ArrayLiteral(ArrayLiteral { elements, .. }) => {
                if elements.len() != 1 {
                    return None;
                }
                if let Element::Expression(ref exp) = elements[0] {
                    Some(vec![Attribute::Metadata(exp.clone())])
                } else {
                    None
                }
            },
            Self::ComputedMember(ComputedMemberExpression { base, key, .. }) => {
                let mut a = base.to_metadata()?;
                if matches!(key.as_ref(), Self::Sequence(_)) {
                    return None;
                }
                a.push(Attribute::Metadata(key.clone()));
                Some(a)
            },
            _ => None,
        }
    }
}