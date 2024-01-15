/// Indicates a fatal syntax error that leads parsing
/// to finish without a resulting node.
/// 
/// This parser is intolerant in general,
/// thus resulting in a `ParsingFailure` for almost any syntax error.
#[derive(Copy, Clone, Debug)]
pub struct ParsingFailure;