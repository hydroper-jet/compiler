use crate::ns::*;

/// Jet Language reserved word validation.
pub struct JetReservedWord;

impl JetReservedWord {
    /// Checks if an *IdentifierName* is a reserved word.
    pub fn test(name: &str) -> bool {
        JetReservedWord::token(name).is_some()
    }

    /// Attempts to convert an *IdentifierName* into a reserved word token.
    pub fn token(name: &str) -> Option<Token> {
        match name.len() {
            1 => None,
            2 => {
                match name {
                    "as" => Some(Token::As),
                    "do" => Some(Token::Do),
                    "if" => Some(Token::If),
                    "in" => Some(Token::In),
                    "is" => Some(Token::Is),
                    _ => None,
                }
            },
            3 => {
                match name {
                    "for" => Some(Token::For),
                    "new" => Some(Token::New),
                    "not" => Some(Token::Not),
                    "try" => Some(Token::Try),
                    "use" => Some(Token::Use),
                    "var" => Some(Token::Var),
                    _ => None,
                }
            },
            4 => {
                match name {
                    "case" => Some(Token::Case),
                    "else" => Some(Token::Else),
                    "this" => Some(Token::This),
                    "true" => Some(Token::True),
                    "void" => Some(Token::Void),
                    "with" => Some(Token::With),
                    _ => None,
                }
            },
            5 => {
                match name {
                    "await" => Some(Token::Await),
                    "break" => Some(Token::Break),
                    "catch" => Some(Token::Catch),
                    "class" => Some(Token::Class),
                    "const" => Some(Token::Const),
                    "false" => Some(Token::False),
                    "super" => Some(Token::Super),
                    "throw" => Some(Token::Throw),
                    "while" => Some(Token::While),
                    "yield" => Some(Token::Yield),
                    _ => None,
                }
            },
            6 => {
                match name {
                    "delete" => Some(Token::Delete),
                    "import" => Some(Token::Import),
                    "public" => Some(Token::Public),
                    "return" => Some(Token::Return),
                    "switch" => Some(Token::Switch),
                    "typeof" => Some(Token::Typeof),
                    _ => None,
                }
            },
            7 => {
                match name {
                    "default" => Some(Token::Default),
                    "extends" => Some(Token::Extends),
                    "finally" => Some(Token::Finally),
                    "package" => Some(Token::Package),
                    "private" => Some(Token::Private),
                    _ => None,
                }
            },
            8 => {
                match name {
                    "continue" => Some(Token::Continue),
                    "function" => Some(Token::Function),
                    "internal" => Some(Token::Internal),
                    _ => None,
                }
            },
            9 => {
                match name {
                    "interface" => Some(Token::Interface),
                    "protected" => Some(Token::Protected),
                    "undefined" => Some(Token::Undefined),
                    _ => None,
                }
            },
            10 => {
                match name {
                    "implements" => Some(Token::Implements),
                    _ => None,
                }
            },
            _ => None,
        }
    }
}