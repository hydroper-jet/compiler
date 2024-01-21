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
        DiagnosticKind::ExpectedStringLiteral.id() => "Expected string literal before {1}".into(),
        DiagnosticKind::DuplicateAttribute.id() => "Duplicate attribute".into(),
        DiagnosticKind::DuplicateVisibility.id() => "Duplicate visibility".into(),
        DiagnosticKind::ExpectedDirectiveKeyword.id() => "Expected directive keyword".into(),
        DiagnosticKind::UnallowedAttribute.id() => "Unallowed attribute".into(),
        DiagnosticKind::UseDirectiveMustContainPublic.id() => "Use directive must contain the 'public' attribute".into(),
        DiagnosticKind::MalformedEnumMember.id() => "Malformed enumeration member".into(),
        DiagnosticKind::FunctionMayNotBeGenerator.id() => "Function may not be generator".into(),
        DiagnosticKind::FunctionMayNotBeAsynchronous.id() => "Function may not be asynchronous".into(),
        DiagnosticKind::FunctionMustNotContainBody.id() => "Function must not contain body".into(),
        DiagnosticKind::FunctionMustContainBody.id() => "Function must contain body".into(),
        DiagnosticKind::FunctionMustNotContainAnnotations.id() => "Function must not contain annotations".into(),
        DiagnosticKind::NestedClassesNotAllowed.id() => "Nested classes are not allowed".into(),
        DiagnosticKind::DirectiveNotAllowedInInterface.id() => "Directive not allowed in interface".into(),
        DiagnosticKind::FailedParsingJetDocTag.id() => "Failed parsing contents of JetDoc tag: '@{1}'".into(),
        DiagnosticKind::UnrecognizedJetDocTag.id() => "Unrecognized JetDoc tag: '@{1}'".into(),
        // DiagnosticKind::K.id() => "".into(),
    };
}