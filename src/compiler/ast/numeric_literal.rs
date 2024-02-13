use crate::ns::*;
use num_bigint::BigInt;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use conv::ValueFrom;
use num_traits::ToPrimitive;

#[derive(Clone, Serialize, Deserialize)]
pub struct NumericLiteral {
    pub location: Location,
    /// The numeric value in character representation. Such representation may be parsed
    /// through data type specific methods such as [`NumericLiteral::parse_double()`].
    pub value: String,
}

impl NumericLiteral {
    /// Parses a double-precision floating point either in
    /// decimal, binary (`0b`) or hexadecimal (`0x`) notation.
    pub fn parse_double(&self, negative: bool) -> Result<f64, ParsingFailure> {
        let s = self.value.replace('_', "");
        if s.starts_with('0') {
            if s[1..].starts_with('x') || s[1..].starts_with('X') {
                let n = u64::from_str_radix(&(if negative { "-" } else { "" }.to_owned() + &s[2..]), 16);
                return n.map_err(|_| ParsingFailure)
                    .and_then(|n| f64::value_from(n).map_err(|_| ParsingFailure));
            } else if s[1..].starts_with('b') || s[1..].starts_with('B') {
                let n = u64::from_str_radix(&(if negative { "-" } else { "" }.to_owned() + &s[2..]), 2);
                return n.map_err(|_| ParsingFailure)
                    .and_then(|n| f64::value_from(n).map_err(|_| ParsingFailure));
            }
        }
        f64::from_str(&(if negative { "-" } else { "" }.to_owned() + &s)).map_err(|_| ParsingFailure)
    }

    /// Parses a single-precision floating point either in
    /// decimal, binary (`0b`) or hexadecimal (`0x`) notation.
    pub fn parse_single(&self, negative: bool) -> Result<f32, ParsingFailure> {
        let s = self.value.replace('_', "");
        if s.starts_with('0') {
            if s[1..].starts_with('x') || s[1..].starts_with('X') {
                let n = u64::from_str_radix(&(if negative { "-" } else { "" }.to_owned() + &s[2..]), 16);
                return n.map_err(|_| ParsingFailure)
                    .and_then(|n| f32::value_from(n).map_err(|_| ParsingFailure));
            } else if s[1..].starts_with('b') || s[1..].starts_with('B') {
                let n = u64::from_str_radix(&(if negative { "-" } else { "" }.to_owned() + &s[2..]), 2);
                return n.map_err(|_| ParsingFailure)
                    .and_then(|n| f32::value_from(n).map_err(|_| ParsingFailure));
            }
        }
        f32::from_str(&(if negative { "-" } else { "" }.to_owned() + &s)).map_err(|_| ParsingFailure)
    }

    /// Parses a signed long either in
    /// decimal, binary (`0b`) or hexadecimal (`0x`) notation.
    pub fn parse_long(&self, negative: bool) -> Result<i64, ParsingFailure> {
        let s = self.value.replace('_', "");
        if s.starts_with('0') {
            if s[1..].starts_with('x') || s[1..].starts_with('X') {
                let n = i64::from_str_radix(&(if negative { "-" } else { "" }.to_owned() + &s[2..]), 16);
                return n.map_err(|_| ParsingFailure);
            } else if s[1..].starts_with('b') || s[1..].starts_with('B') {
                let n = i64::from_str_radix(&(if negative { "-" } else { "" }.to_owned() + &s[2..]), 2);
                return n.map_err(|_| ParsingFailure);
            }
        }
        i64::from_str(&s).map_err(|_| ParsingFailure)
    }

    /// Parses a big integer either in
    /// decimal, binary (`0b`) or hexadecimal (`0x`) notation.
    pub fn parse_big_int(&self, negative: bool) -> Result<BigInt, ParsingFailure> {
        let s = self.value.replace('_', "");
        if s.starts_with('0') {
            if s[1..].starts_with('x') || s[1..].starts_with('X') {
                let mut digits: Vec<u8> = vec![];
                for ch in s[2..].chars() {
                    digits.push(CharacterValidator::hex_digit_mv(ch).unwrap().to_u8().unwrap());
                }
                let n = BigInt::from_radix_be(if negative { num_bigint::Sign::Minus } else { num_bigint::Sign::Plus }, &digits, 16);
                return n.map_or(Err(ParsingFailure), |n| Ok(n));
            } else if s[1..].starts_with('b') || s[1..].starts_with('B') {
                let mut digits: Vec<u8> = vec![];
                for ch in s[2..].chars() {
                    digits.push(CharacterValidator::bin_digit_mv(ch).unwrap().to_u8().unwrap());
                }
                let n = BigInt::from_radix_be(if negative { num_bigint::Sign::Minus } else { num_bigint::Sign::Plus }, &digits, 2);
                return n.map_or(Err(ParsingFailure), |n| Ok(n));
            }
        }
        BigInt::from_str(&s).map_err(|_| ParsingFailure)
    }

    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier, context: &ExpressionVerifyContext) -> Result<Option<Symbol>, DeferVerificationError> {
        if let Some(t) = context.context_type.as_ref() {
            let t_non_null = t.non_null_type();
            if verifier.host.is_numeric_type(&t_non_null) {
                let n = self.parse_with_static_type(&verifier.host, &t_non_null, context);
                if n.is_err() {
                    verifier.add_syntax_error(&self.location, DiagnosticKind::FailedParsingNumericLiteral, diagnostic_arguments![]);
                    return Ok(None);
                }
                return Ok(Some(verifier.host.factory().create_number_constant(n.unwrap(), t)));
            }
        }
        let t = verifier.host.number_type();
        let n = self.parse_with_static_type(&verifier.host, &t, context);
        if n.is_err() {
            verifier.add_syntax_error(&self.location, DiagnosticKind::FailedParsingNumericLiteral, diagnostic_arguments![]);
            return Ok(None);
        }
        return Ok(Some(verifier.host.factory().create_number_constant(n.unwrap(), &t)));
    }

    fn parse_with_static_type(&self, host: &Rc<SymbolHost>, static_type: &Symbol, context: &ExpressionVerifyContext) -> Result<AbstractRangeNumber, ParsingFailure> {
        if static_type == &host.number_type() {
            Ok(AbstractRangeNumber::Number(self.parse_double(context.preceded_by_negative)?))
        } else if static_type == &host.single_type() {
            Ok(AbstractRangeNumber::Single(self.parse_single(context.preceded_by_negative)?))
        } else if static_type == &host.long_type() {
            Ok(AbstractRangeNumber::Long(self.parse_long(context.preceded_by_negative)?))
        } else {
            assert!(static_type == &host.big_int_type());
            Ok(AbstractRangeNumber::BigInt(self.parse_big_int(context.preceded_by_negative)?))
        }
    }
}

mod tests {
    #[allow(unused)]
    use crate::ns::*;
    #[allow(unused)]
    use std::rc::Rc;

    #[test]
    fn test_minimum_maximum() {
        // Long.MIN_VALUE
        let literal = NumericLiteral {
            location: Location::with_offset(&Rc::new(CompilationUnit::default()), 0),
            value: "0x8000_0000_0000_0000".to_owned(),
        };
        assert_eq!(i64::MIN, literal.parse_long(true).unwrap());

        // Long.MAX_VALUE
        let literal = NumericLiteral {
            location: Location::with_offset(&Rc::new(CompilationUnit::default()), 0),
            value: "0x7FFF_FFFF_FFFF_FFFF".to_owned(),
        };
        assert_eq!(i64::MAX, literal.parse_long(false).unwrap());
    }
}