use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Clone, Serialize, Deserialize)]
pub struct XmlExpression {
    pub location: Location,
    pub element: Rc<XmlElement>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct XmlMarkupExpression {
    pub location: Location,
    pub markup: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct XmlListExpression {
    pub location: Location,
    pub content: Vec<Rc<XmlElementContent>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct XmlElement {
    pub location: Location,
    pub name: XmlTagName,
    pub attributes: Vec<Rc<XmlAttribute>>,
    pub attribute_expression: Option<Rc<Expression>>,
    pub content: Option<Vec<Rc<XmlElementContent>>>,
    pub closing_name: Option<XmlTagName>,
}

impl XmlElement {
    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier) -> Result<(), DeferVerificationError> {
        if let XmlTagName::Expression(exp) = &self.name {
            verifier.verify_expression(exp, &ExpressionVerifyContext { ..default() })?;
        }
        for attr in &self.attributes {
            if let XmlAttributeValue::Expression(exp) = &attr.value {
                verifier.verify_expression(exp, &ExpressionVerifyContext { ..default() })?;
            }
        }
        if let Some(exp) = &self.attribute_expression {
            verifier.verify_expression(exp, &ExpressionVerifyContext { ..default() })?;
        }
        if let Some(content_list) = &self.content {
            for content in content_list {
                content.verify(verifier)?;
            }
        }
        if let Some(XmlTagName::Expression(exp)) = &self.closing_name {
            verifier.verify_expression(exp, &ExpressionVerifyContext { ..default() })?;
        }
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum XmlTagName {
    Name((String, Location)),
    Expression(Rc<Expression>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct XmlAttribute {
    pub location: Location,
    pub name: (String, Location),
    pub value: XmlAttributeValue,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum XmlAttributeValue {
    Value((String, Location)),
    Expression(Rc<Expression>),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum XmlElementContent {
    XmlText((String, Location)),
    XmlMarkup((String, Location)),
    XmlElement(Rc<XmlElement>),
    Expression(Rc<Expression>),
}

impl XmlElementContent {
    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier) -> Result<(), DeferVerificationError> {
        match self {
            Self::XmlElement(el) => {
                el.verify(verifier)?;
                Ok(())
            },
            Self::Expression(exp) => {
                verifier.verify_expression(exp, &ExpressionVerifyContext { ..default() })?;
                Ok(())
            },
            _ => Ok(()),
        }
    }
}