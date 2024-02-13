use crate::ns::*;
use num_traits::{FromPrimitive, One, Zero};
use num_bigint::BigInt;
use std::ops::{BitXor, BitOr};

#[derive(Clone, PartialEq)]
pub enum AbstractRangeNumber {
    Single(f32),
    Number(f64),
    BigInt(BigInt),
    Long(i64),
}

impl BitXor for AbstractRangeNumber {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        match self {
            Self::Single(v) => {
                let Self::Single(rhs) = rhs else { panic!(); };
                Self::Single(f32::from_u32(unsafe { v.to_int_unchecked::<u32>() } ^ unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0.0))
            },
            Self::Number(v) => {
                let Self::Number(rhs) = rhs else { panic!(); };
                Self::Number(f64::from_u32(unsafe { v.to_int_unchecked::<u32>() } ^ unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0.0))
            },
            Self::BigInt(v) => {
                let Self::BigInt(ref rhs) = rhs else { panic!(); };
                Self::BigInt(v.clone() ^ rhs.clone())
            },
            Self::Long(v) => {
                let Self::Long(rhs) = rhs else { panic!(); };
                Self::Long(v ^ rhs)
            },
        }
    }
}

impl BitOr for AbstractRangeNumber {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        match self {
            Self::Single(v) => {
                let Self::Single(rhs) = rhs else { panic!(); };
                Self::Single(f32::from_u32(unsafe { v.to_int_unchecked::<u32>() } | unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0.0))
            },
            Self::Number(v) => {
                let Self::Number(rhs) = rhs else { panic!(); };
                Self::Number(f64::from_u32(unsafe { v.to_int_unchecked::<u32>() } | unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0.0))
            },
            Self::BigInt(v) => {
                let Self::BigInt(ref rhs) = rhs else { panic!(); };
                Self::BigInt(v.clone() | rhs.clone())
            },
            Self::Long(v) => {
                let Self::Long(rhs) = rhs else { panic!(); };
                Self::Long(v | rhs)
            },
        }
    }
}

impl AbstractRangeNumber {
    pub fn zero(type_symbol: &Symbol, host: &SymbolHost) -> Self {
        if type_symbol == &host.number_type() {
            Self::Number(0.0)
        } else if type_symbol == &host.long_type() {
            Self::Long(0)
        } else if type_symbol == &host.big_int_type() {
            Self::BigInt(BigInt::zero())
        } else if type_symbol == &host.single_type() {
            Self::Single(0.0)
        } else {
            panic!()
        }
    }

    pub fn one(type_symbol: &Symbol, host: &SymbolHost) -> Self {
        if type_symbol == &host.number_type() {
            Self::Number(1.0)
        } else if type_symbol == &host.long_type() {
            Self::Long(1)
        } else if type_symbol == &host.big_int_type() {
            Self::BigInt(BigInt::one())
        } else if type_symbol == &host.single_type() {
            Self::Single(1.0)
        } else {
            panic!()
        }
    }

    pub fn minimum_value(type_symbol: &Symbol, host: &SymbolHost) -> Self {
        if type_symbol == &host.number_type() {
            Self::Number(f64::NEG_INFINITY)
        } else if type_symbol == &host.long_type() {
            Self::Long(i64::MIN)
        } else if type_symbol == &host.big_int_type() {
            panic!("BigInt has no minimum value")
        } else if type_symbol == &host.single_type() {
            Self::Single(f32::NEG_INFINITY)
        } else {
            panic!()
        }
    }

    pub fn maximum_value(type_symbol: &Symbol, host: &SymbolHost) -> Self {
        if type_symbol == &host.number_type() {
            Self::Number(f64::INFINITY)
        } else if type_symbol == &host.long_type() {
            Self::Long(i64::MAX)
        } else if type_symbol == &host.big_int_type() {
            panic!("BigInt has no maximum value")
        } else if type_symbol == &host.single_type() {
            Self::Single(f32::INFINITY)
        } else {
            panic!()
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Self::Single(v) => v == &0.0,
            Self::Number(v) => v == &0.0,
            Self::BigInt(v) => v.is_zero(),
            Self::Long(v) => v == &0,
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            Self::Single(v) => v == &1.0,
            Self::Number(v) => v == &1.0,
            Self::BigInt(v) => v.is_one(),
            Self::Long(v) => v == &1,
        }
    }

    pub fn multiply_per_two(&self) -> Self {
        match self {
            Self::Single(v) => Self::Single(v * 2.0),
            Self::Number(v) => Self::Number(v * 2.0),
            Self::BigInt(v) => Self::BigInt(v * 2),
            Self::Long(v) => Self::Long(v * 2),
        }
    }

    pub fn increase_by_one(&self) -> Self {
        match self {
            Self::Single(v) => Self::Single(v + 1.0),
            Self::Number(v) => Self::Number(v + 1.0),
            Self::BigInt(v) => Self::BigInt(v + 1),
            Self::Long(v) => Self::Long(v + 1),
        }
    }

    /// Performs bitwise OR if `value` is true or erases bits with the `erase_bits()` method otherwise.
    pub fn apply_bits(&self, bits: &Self, value: bool) -> Self {
        if value { self.clone() | bits.clone() } else { self.erase_bits(bits) }
    }

    /// Erases bits if all of such bits are included in the base value.
    pub fn erase_bits(&self, bits: &Self) -> Self {
        if self.includes_bits(bits) { self.clone() ^ bits.clone() } else { self.clone() }
    }

    pub fn includes_bits(&self, rhs: &Self) -> bool {
        match self {
            Self::Single(v) => {
                let Self::Single(rhs) = rhs else { panic!(); };
                (unsafe { v.to_int_unchecked::<u32>() } & unsafe { rhs.to_int_unchecked::<u32>() } != 0)
            },
            Self::Number(v) => {
                let Self::Number(rhs) = rhs else { panic!(); };
                (unsafe { v.to_int_unchecked::<u32>() } & unsafe { rhs.to_int_unchecked::<u32>() } != 0)
            },
            Self::BigInt(v) => {
                let Self::BigInt(ref rhs) = rhs else { panic!(); };
                !(v.clone() & rhs.clone()).is_zero()
            },
            Self::Long(v) => {
                let Self::Long(rhs) = rhs else { panic!(); };
                v & rhs != 0
            },
        }
    }

    pub fn is_power_of_two(&self) -> bool {
        // Based on https://stackoverflow.com/a/600306
        match self {
            Self::Single(v) => {
                let v = unsafe { v.to_int_unchecked::<u32>() };
                (v != 0) && ((v & (v - 1)) == 0)
            },
            Self::Number(v) => {
                let v = unsafe { v.to_int_unchecked::<u32>() };
                (v != 0) && ((v & (v - 1)) == 0)
            },
            Self::BigInt(v) => {
                !v.is_zero() && ((v & (v - BigInt::one())).is_zero())
            },
            Self::Long(v) => (v != &0) && ((v & (v - 1)) == 0),
        }
    }

    pub fn convert_type(&self, target_type: &Symbol, host: &SymbolHost) -> Self {
        let number_type = host.number_type();
        let single_type = host.single_type();
        let long_type = host.long_type();
        let big_int_type = host.big_int_type();

        if target_type == &number_type {
            match self {
                Self::Single(v) => Self::Number(*v as f64),
                Self::Number(v) => Self::Number(*v),
                Self::BigInt(v) => {
                    let v: Result<u32, _> = v.try_into();
                    Self::Number(v.map(|v| v as f64).unwrap_or(f64::NAN))
                },
                Self::Long(v) => {
                    let v: Result<i32, _> = (*v).try_into();
                    Self::Number(v.map(|v| v as f64).unwrap_or(f64::NAN))
                },
            }
        } else if target_type == &single_type {
            match self {
                Self::Single(v) => Self::Single(*v),
                Self::Number(v) => Self::Single(*v as f32),
                Self::BigInt(v) => {
                    let v: Result<u32, _> = v.try_into();
                    Self::Single(v.map(|v| v as f32).unwrap_or(f32::NAN))
                },
                Self::Long(v) => {
                    let v: Result<u32, _> = (*v).try_into();
                    Self::Single(v.map(|v| v as f32).unwrap_or(f32::NAN))
                },
            }
        } else if target_type == &long_type {
            match self {
                Self::Single(v) => Self::Long(unsafe { v.to_int_unchecked() }),
                Self::Number(v) => Self::Long(unsafe { v.to_int_unchecked() }),
                Self::BigInt(v) => Self::Long(v.try_into().unwrap_or(0)),
                Self::Long(v) => Self::Long((*v).try_into().unwrap_or(0)),
            }
        } else if target_type == &big_int_type {
            match self {
                Self::Single(v) => Self::BigInt(BigInt::from_f32(*v).unwrap_or(BigInt::zero())),
                Self::Number(v) => Self::BigInt(BigInt::from_f64(*v).unwrap_or(BigInt::zero())),
                Self::BigInt(v) => Self::BigInt(v.clone()),
                Self::Long(v) => Self::BigInt((*v).into()),
            }
        } else {
            panic!()
        }
    }

    pub fn is_nan(&self) -> bool {
        match self {
            Self::Number(f) => f.is_nan(),
            Self::Single(f) => f.is_nan(),
            _ => false,
        }
    }

    pub fn is_negative_infinity(&self) -> bool {
        match self {
            Self::Number(f) => f == &f64::NEG_INFINITY,
            Self::Single(f) => f == &f32::NEG_INFINITY,
            _ => false,
        }
    }

    pub fn is_positive_infinity(&self) -> bool {
        match self {
            Self::Number(f) => f == &f64::INFINITY,
            Self::Single(f) => f == &f32::INFINITY,
            _ => false,
        }
    }
}