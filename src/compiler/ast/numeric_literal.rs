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
    pub fn parse_double(&self) -> Result<f64, ParsingFailure> {
        let s = self.value.replace('_', "");
        if s.starts_with('0') {
            if s[1..].starts_with('x') || s[1..].starts_with('X') {
                let n = u64::from_str_radix(&s[2..], 16);
                return n.map_err(|_| ParsingFailure)
                    .and_then(|n| f64::value_from(n).map_err(|_| ParsingFailure));
            } else if s[1..].starts_with('b') || s[1..].starts_with('B') {
                let n = u64::from_str_radix(&s[2..], 2);
                return n.map_err(|_| ParsingFailure)
                    .and_then(|n| f64::value_from(n).map_err(|_| ParsingFailure));
            }
        }
        f64::from_str(&s).map_err(|_| ParsingFailure)
    }
}