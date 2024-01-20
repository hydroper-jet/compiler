use std::collections::HashMap;
use lazy_static::lazy_static;
use maplit::hashmap;
use crate::ns::*;

lazy_static! {
    pub static ref DATA: HashMap<i32, String> = hashmap! {
        // DiagnosticKind::K.id() => "".into(),
        DiagnosticKind::UnexpectedOrInvalidToken.id() => "Unexpected or invalid token".into(),
        DiagnosticKind::UnexpectedEnd.id() => "Unexpected end of program".into(),
        DiagnosticKind::UnallowedNumericSuffix.id() => "Unallowed numeric suffix".into(),
        DiagnosticKind::UnallowedLineBreak.id() => "Unallowed line break".into(),
        DiagnosticKind::Expected.id() => "Expected {1} before {2}".into(),
        DiagnosticKind::ExpectedIdentifier.id() => "Expected identifier before {1}".into(),
        DiagnosticKind::ExpectedExpression.id() => "Expected expression before {1}".into(),
        DiagnosticKind::ExpectedXmlName.id() => "Expected XML name before {1}".into(),
        DiagnosticKind::ExpectedXmlAttributeValue.id() => "Expected XML attribute value before {1}".into(),
        DiagnosticKind::IllegalNullishCoalescingLeftOperand.id() => "Illegal nullish ooalescing left operand".into(),
        DiagnosticKind::WrongParameterPosition.id() => "Wrong parameter position".into(),
        DiagnosticKind::DuplicateRestParameter.id() => "Duplicate rest parameter".into(),
        DiagnosticKind::NotAllowedHere.id() => "{1} not allowed here".into(),
        DiagnosticKind::MalformedRestParameter.id() => "Malformed rest parameter".into(),
        DiagnosticKind::IllegalForInInitializer.id() => "Illegal 'for..in' initializer".into(),
        DiagnosticKind::MultipleForInBindings.id() => "Multiple 'for..in' bindings are not allowed".into(),
        DiagnosticKind::UndefinedLabel.id() => "Undefined label '{1}'".into(),
        DiagnosticKind::IllegalContinue.id() => "Illegal continue statement".into(),
        DiagnosticKind::IllegalBreak.id() => "Illegal break statement".into(),
        DiagnosticKind::ExpressionMustNotFollowLineBreak.id() => "Expression must not follow line break".into(),
        DiagnosticKind::TokenMustNotFollowLineBreak.id() => "Token must not follow line break".into(),
        // DiagnosticKind::K.id() => "".into(),
    };
}