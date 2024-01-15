use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;
use std::str::FromStr;
use conv::ValueFrom;

#[derive(Clone, Serialize, Deserialize)]
pub struct NumericLiteral {
    pub location: Location,
    /// The numeric value is in character representation. Such representation may be parsed
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

    /// Parses an unsigned long either in
    /// decimal, binary (`0b`) or hexadecimal (`0x`) notation.
    pub fn parse_unsigned_long(&self, negative: bool) -> Result<u64, ParsingFailure> {
        let s = self.value.replace('_', "");
        if s.starts_with('0') {
            if s[1..].starts_with('x') || s[1..].starts_with('X') {
                let n = u64::from_str_radix(&(if negative { "-" } else { "" }.to_owned() + &s[2..]), 16);
                return n.map_err(|_| ParsingFailure);
            } else if s[1..].starts_with('b') || s[1..].starts_with('B') {
                let n = u64::from_str_radix(&(if negative { "-" } else { "" }.to_owned() + &s[2..]), 2);
                return n.map_err(|_| ParsingFailure);
            }
        }
        u64::from_str(&(if negative { "-" } else { "" }.to_owned() + &s)).map_err(|_| ParsingFailure)
    }
}

mod tests {
    use crate::ns::*;
    use std::rc::Rc;

    #[test]
    fn test_minimum_maximum() {
        // Long.MIN_VALUE
        let literal = NumericLiteral {
            location: Location::with_line_and_offset(&Rc::new(CompilationUnit::default()), 1, 0),
            value: "0x8000_0000_0000_0000".to_owned(),
        };
        assert_eq!(i64::MIN, literal.parse_long(true).unwrap());

        // Long.MAX_VALUE
        let literal = NumericLiteral {
            location: Location::with_line_and_offset(&Rc::new(CompilationUnit::default()), 1, 0),
            value: "0x7FFF_FFFF_FFFF_FFFF".to_owned(),
        };
        assert_eq!(i64::MAX, literal.parse_long(false).unwrap());
    }
}