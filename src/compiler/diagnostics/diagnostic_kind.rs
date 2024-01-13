#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum DiagnosticKind {
    UnexpectedOrInvalidToken = 1024,
    UnexpectedEnd = 1025,
    UnallowedNumericSuffix = 1026,
    UnallowedLineBreak = 1027,
    Expected = 1028,
    ExpectedIdentifier = 1029,
    ExpectedExpression = 1030,
    ExpectedXmlName = 1031,
    ExpectedXmlAttributeValue = 1032,
}

impl DiagnosticKind {
    pub fn id(&self) -> i32 {
        *self as i32
    }
}