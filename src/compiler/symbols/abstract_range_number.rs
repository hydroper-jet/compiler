use crate::ns::*;
use num_traits::{FromPrimitive, One, Zero};
use num_bigint::BigInt;
use std::ops::BitOr;

#[derive(Clone, PartialEq)]
pub enum AbstractRangeNumber {
    Single(f32),
    Number(f64),
    BigInt(BigInt),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    UnsignedByte(u8),
    UnsignedShort(u16),
    UnsignedInt(u32),
    UnsignedLong(u64),
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
            Self::Byte(v) => {
                let Self::Byte(rhs) = rhs else { panic!(); };
                Self::Byte(v | rhs)
            },
            Self::Short(v) => {
                let Self::Short(rhs) = rhs else { panic!(); };
                Self::Short(v | rhs)
            },
            Self::Int(v) => {
                let Self::Int(rhs) = rhs else { panic!(); };
                Self::Int(v | rhs)
            },
            Self::Long(v) => {
                let Self::Long(rhs) = rhs else { panic!(); };
                Self::Long(v | rhs)
            },
            Self::UnsignedByte(v) => {
                let Self::UnsignedByte(rhs) = rhs else { panic!(); };
                Self::UnsignedByte(v | rhs)
            },
            Self::UnsignedShort(v) => {
                let Self::UnsignedShort(rhs) = rhs else { panic!(); };
                Self::UnsignedShort(v | rhs)
            },
            Self::UnsignedInt(v) => {
                let Self::UnsignedInt(rhs) = rhs else { panic!(); };
                Self::UnsignedInt(v | rhs)
            },
            Self::UnsignedLong(v) => {
                let Self::UnsignedLong(rhs) = rhs else { panic!(); };
                Self::UnsignedLong(v | rhs)
            },
        }
    }
}

impl AbstractRangeNumber {
    pub fn zero(type_symbol: &Symbol, host: &mut SymbolHost) -> Self {
        if type_symbol == &host.number_type() {
            Self::Number(0.0)
        } else if type_symbol == &host.int_type() {
            Self::Int(0)
        } else if type_symbol == &host.unsigned_int_type() {
            Self::UnsignedInt(0)
        } else if type_symbol == &host.byte_type() {
            Self::Byte(0)
        } else if type_symbol == &host.unsigned_byte_type() {
            Self::UnsignedByte(0)
        } else if type_symbol == &host.short_type() {
            Self::Short(0)
        } else if type_symbol == &host.unsigned_short_type() {
            Self::UnsignedShort(0)
        } else if type_symbol == &host.long_type() {
            Self::Long(0)
        } else if type_symbol == &host.unsigned_long_type() {
            Self::UnsignedLong(0)
        } else if type_symbol == &host.big_int_type() {
            Self::BigInt(BigInt::zero())
        } else if type_symbol == &host.single_type() {
            Self::Single(0.0)
        } else {
            panic!()
        }
    }

    pub fn one(type_symbol: &Symbol, host: &mut SymbolHost) -> Self {
        if type_symbol == &host.number_type() {
            Self::Number(1.0)
        } else if type_symbol == &host.int_type() {
            Self::Int(1)
        } else if type_symbol == &host.unsigned_int_type() {
            Self::UnsignedInt(1)
        } else if type_symbol == &host.byte_type() {
            Self::Byte(1)
        } else if type_symbol == &host.unsigned_byte_type() {
            Self::UnsignedByte(1)
        } else if type_symbol == &host.short_type() {
            Self::Short(1)
        } else if type_symbol == &host.unsigned_short_type() {
            Self::UnsignedShort(1)
        } else if type_symbol == &host.long_type() {
            Self::Long(1)
        } else if type_symbol == &host.unsigned_long_type() {
            Self::UnsignedLong(1)
        } else if type_symbol == &host.big_int_type() {
            Self::BigInt(BigInt::one())
        } else if type_symbol == &host.single_type() {
            Self::Single(1.0)
        } else {
            panic!()
        }
    }

    pub fn minimum_value(type_symbol: &Symbol, host: &mut SymbolHost) -> Self {
        if type_symbol == &host.number_type() {
            Self::Number(f64::NEG_INFINITY)
        } else if type_symbol == &host.int_type() {
            Self::Int(i32::MIN)
        } else if type_symbol == &host.unsigned_int_type() {
            Self::UnsignedInt(u32::MIN)
        } else if type_symbol == &host.byte_type() {
            Self::Byte(i8::MIN)
        } else if type_symbol == &host.unsigned_byte_type() {
            Self::UnsignedByte(u8::MIN)
        } else if type_symbol == &host.short_type() {
            Self::Short(i16::MIN)
        } else if type_symbol == &host.unsigned_short_type() {
            Self::UnsignedShort(u16::MIN)
        } else if type_symbol == &host.long_type() {
            Self::Long(i64::MIN)
        } else if type_symbol == &host.unsigned_long_type() {
            Self::UnsignedLong(u64::MIN)
        } else if type_symbol == &host.big_int_type() {
            panic!("BigInt has no minimum value")
        } else if type_symbol == &host.single_type() {
            Self::Single(f32::NEG_INFINITY)
        } else {
            panic!()
        }
    }

    pub fn maximum_value(type_symbol: &Symbol, host: &mut SymbolHost) -> Self {
        if type_symbol == &host.number_type() {
            Self::Number(f64::INFINITY)
        } else if type_symbol == &host.int_type() {
            Self::Int(i32::MAX)
        } else if type_symbol == &host.unsigned_int_type() {
            Self::UnsignedInt(u32::MAX)
        } else if type_symbol == &host.byte_type() {
            Self::Byte(i8::MAX)
        } else if type_symbol == &host.unsigned_byte_type() {
            Self::UnsignedByte(u8::MAX)
        } else if type_symbol == &host.short_type() {
            Self::Short(i16::MAX)
        } else if type_symbol == &host.unsigned_short_type() {
            Self::UnsignedShort(u16::MAX)
        } else if type_symbol == &host.long_type() {
            Self::Long(i64::MAX)
        } else if type_symbol == &host.unsigned_long_type() {
            Self::UnsignedLong(u64::MAX)
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
            Self::Byte(v) => v == &0,
            Self::Short(v) => v == &0,
            Self::Int(v) => v == &0,
            Self::Long(v) => v == &0,
            Self::UnsignedByte(v) => v == &0,
            Self::UnsignedShort(v) => v == &0,
            Self::UnsignedInt(v) => v == &0,
            Self::UnsignedLong(v) => v == &0,
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            Self::Single(v) => v == &1.0,
            Self::Number(v) => v == &1.0,
            Self::BigInt(v) => v.is_one(),
            Self::Byte(v) => v == &1,
            Self::Short(v) => v == &1,
            Self::Int(v) => v == &1,
            Self::Long(v) => v == &1,
            Self::UnsignedByte(v) => v == &1,
            Self::UnsignedShort(v) => v == &1,
            Self::UnsignedInt(v) => v == &1,
            Self::UnsignedLong(v) => v == &1,
        }
    }

    pub fn multiply_per_two(&self) -> Self {
        match self {
            Self::Single(v) => Self::Single(v * 2.0),
            Self::Number(v) => Self::Number(v * 2.0),
            Self::BigInt(v) => Self::BigInt(v * 2),
            Self::Byte(v) => Self::Byte(v * 2),
            Self::Short(v) => Self::Short(v * 2),
            Self::Int(v) => Self::Int(v * 2),
            Self::Long(v) => Self::Long(v * 2),
            Self::UnsignedByte(v) => Self::UnsignedByte(v * 2),
            Self::UnsignedShort(v) => Self::UnsignedShort(v * 2),
            Self::UnsignedInt(v) => Self::UnsignedInt(v * 2),
            Self::UnsignedLong(v) => Self::UnsignedLong(v * 2),
        }
    }

    pub fn increase_by_one(&self) -> Self {
        match self {
            Self::Single(v) => Self::Single(v + 1.0),
            Self::Number(v) => Self::Number(v + 1.0),
            Self::BigInt(v) => Self::BigInt(v + 1),
            Self::Byte(v) => Self::Byte(v * 2),
            Self::Short(v) => Self::Short(v + 1),
            Self::Int(v) => Self::Int(v + 1),
            Self::Long(v) => Self::Long(v + 1),
            Self::UnsignedByte(v) => Self::UnsignedByte(v + 1),
            Self::UnsignedShort(v) => Self::UnsignedShort(v + 1),
            Self::UnsignedInt(v) => Self::UnsignedInt(v + 1),
            Self::UnsignedLong(v) => Self::UnsignedLong(v + 1),
        }
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
            Self::Byte(v) => {
                let Self::Byte(rhs) = rhs else { panic!(); };
                v & rhs != 0
            },
            Self::Short(v) => {
                let Self::Short(rhs) = rhs else { panic!(); };
                v & rhs != 0
            },
            Self::Int(v) => {
                let Self::Int(rhs) = rhs else { panic!(); };
                v & rhs != 0
            },
            Self::Long(v) => {
                let Self::Long(rhs) = rhs else { panic!(); };
                v & rhs != 0
            },
            Self::UnsignedByte(v) => {
                let Self::UnsignedByte(rhs) = rhs else { panic!(); };
                v & rhs != 0
            },
            Self::UnsignedShort(v) => {
                let Self::UnsignedShort(rhs) = rhs else { panic!(); };
                v & rhs != 0
            },
            Self::UnsignedInt(v) => {
                let Self::UnsignedInt(rhs) = rhs else { panic!(); };
                v & rhs != 0
            },
            Self::UnsignedLong(v) => {
                let Self::UnsignedLong(rhs) = rhs else { panic!(); };
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
            Self::Byte(v) => (v != &0) && ((v & (v - 1)) == 0),
            Self::Short(v) => (v != &0) && ((v & (v - 1)) == 0),
            Self::Int(v) => (v != &0) && ((v & (v - 1)) == 0),
            Self::Long(v) => (v != &0) && ((v & (v - 1)) == 0),
            Self::UnsignedByte(v) => (v != &0) && ((v & (v - 1)) == 0),
            Self::UnsignedShort(v) => (v != &0) && ((v & (v - 1)) == 0),
            Self::UnsignedInt(v) => (v != &0) && ((v & (v - 1)) == 0),
            Self::UnsignedLong(v) => (v != &0) && ((v & (v - 1)) == 0),
        }
    }

    pub fn convert_type(&self, target_type: &Symbol, host: &mut SymbolHost) -> Self {
        let number_type = host.number_type();
        let single_type = host.single_type();
        let byte_type = host.byte_type();
        let short_type = host.short_type();
        let int_type = host.int_type();
        let long_type = host.long_type();
        let unsigned_byte_type = host.unsigned_byte_type();
        let unsigned_short_type = host.unsigned_short_type();
        let unsigned_int_type = host.unsigned_int_type();
        let unsigned_long_type = host.unsigned_long_type();
        let big_int_type = host.big_int_type();

        if target_type == &number_type {
            match self {
                Self::Single(v) => Self::Number(*v as f64),
                Self::Number(v) => Self::Number(*v),
                Self::BigInt(v) => {
                    let v: Result<u32, _> = v.try_into();
                    Self::Number(v.map(|v| v as f64).unwrap_or(f64::NAN))
                },
                Self::Byte(v) => Self::Number(*v as f64),
                Self::Short(v) => Self::Number(*v as f64),
                Self::Int(v) => Self::Number(*v as f64),
                Self::Long(v) => {
                    let v: Result<i32, _> = (*v).try_into();
                    Self::Number(v.map(|v| v as f64).unwrap_or(f64::NAN))
                },
                Self::UnsignedByte(v) => Self::Number(*v as f64),
                Self::UnsignedShort(v) => Self::Number(*v as f64),
                Self::UnsignedInt(v) => Self::Number(*v as f64),
                Self::UnsignedLong(v) => {
                    let v: Result<u32, _> = (*v).try_into();
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
                Self::Byte(v) => Self::Single(*v as f32),
                Self::Short(v) => Self::Single(*v as f32),
                Self::Int(v) => {
                    let v: Result<i32, _> = (*v).try_into();
                    Self::Single(v.map(|v| v as f32).unwrap_or(f32::NAN))
                },
                Self::Long(v) => {
                    let v: Result<u32, _> = (*v).try_into();
                    Self::Single(v.map(|v| v as f32).unwrap_or(f32::NAN))
                },
                Self::UnsignedByte(v) => Self::Single(*v as f32),
                Self::UnsignedShort(v) => Self::Single(*v as f32),
                Self::UnsignedInt(v) => {
                    let v: Result<u32, _> = (*v).try_into();
                    Self::Single(v.map(|v| v as f32).unwrap_or(f32::NAN))
                },
                Self::UnsignedLong(v) => {
                    let v: Result<u32, _> = (*v).try_into();
                    Self::Single(v.map(|v| v as f32).unwrap_or(f32::NAN))
                },
            }
        } else if target_type == &byte_type {
            match self {
                Self::Single(v) => Self::Byte(unsafe { v.to_int_unchecked() }),
                Self::Number(v) => Self::Byte(unsafe { v.to_int_unchecked() }),
                Self::BigInt(v) => Self::Byte(v.try_into().unwrap_or(0)),
                Self::Byte(v) => Self::Byte(*v),
                Self::Short(v) => Self::Byte((*v).try_into().unwrap_or(0)),
                Self::Int(v) => Self::Byte((*v).try_into().unwrap_or(0)),
                Self::Long(v) => Self::Byte((*v).try_into().unwrap_or(0)),
                Self::UnsignedByte(v) => Self::Byte((*v).try_into().unwrap_or(0)),
                Self::UnsignedShort(v) => Self::Byte((*v).try_into().unwrap_or(0)),
                Self::UnsignedInt(v) => Self::Byte((*v).try_into().unwrap_or(0)),
                Self::UnsignedLong(v) => Self::Byte((*v).try_into().unwrap_or(0)),
            }
        } else if target_type == &short_type {
            match self {
                Self::Single(v) => Self::Short(unsafe { v.to_int_unchecked() }),
                Self::Number(v) => Self::Short(unsafe { v.to_int_unchecked() }),
                Self::BigInt(v) => Self::Short(v.try_into().unwrap_or(0)),
                Self::Byte(v) => Self::Short(*v as i16),
                Self::Short(v) => Self::Short(*v),
                Self::Int(v) => Self::Short((*v).try_into().unwrap_or(0)),
                Self::Long(v) => Self::Short((*v).try_into().unwrap_or(0)),
                Self::UnsignedByte(v) => Self::Short(*v as i16),
                Self::UnsignedShort(v) => Self::Short((*v).try_into().unwrap_or(0)),
                Self::UnsignedInt(v) => Self::Short((*v).try_into().unwrap_or(0)),
                Self::UnsignedLong(v) => Self::Short((*v).try_into().unwrap_or(0)),
            }
        } else if target_type == &int_type {
            match self {
                Self::Single(v) => Self::Int(unsafe { v.to_int_unchecked() }),
                Self::Number(v) => Self::Int(unsafe { v.to_int_unchecked() }),
                Self::BigInt(v) => Self::Int(v.try_into().unwrap_or(0)),
                Self::Byte(v) => Self::Int(*v as i32),
                Self::Short(v) => Self::Int(*v as i32),
                Self::Int(v) => Self::Int((*v).try_into().unwrap_or(0)),
                Self::Long(v) => Self::Int((*v).try_into().unwrap_or(0)),
                Self::UnsignedByte(v) => Self::Int(*v as i32),
                Self::UnsignedShort(v) => Self::Int(*v as i32),
                Self::UnsignedInt(v) => Self::Int((*v).try_into().unwrap_or(0)),
                Self::UnsignedLong(v) => Self::Int((*v).try_into().unwrap_or(0)),
            }
        } else if target_type == &long_type {
            match self {
                Self::Single(v) => Self::Long(unsafe { v.to_int_unchecked() }),
                Self::Number(v) => Self::Long(unsafe { v.to_int_unchecked() }),
                Self::BigInt(v) => Self::Long(v.try_into().unwrap_or(0)),
                Self::Byte(v) => Self::Long(*v as i64),
                Self::Short(v) => Self::Long(*v as i64),
                Self::Int(v) => Self::Long(*v as i64),
                Self::Long(v) => Self::Long((*v).try_into().unwrap_or(0)),
                Self::UnsignedByte(v) => Self::Long(*v as i64),
                Self::UnsignedShort(v) => Self::Long(*v as i64),
                Self::UnsignedInt(v) => Self::Long(*v as i64),
                Self::UnsignedLong(v) => Self::Long((*v).try_into().unwrap_or(0)),
            }
        } else if target_type == &unsigned_byte_type {
            match self {
                Self::Single(v) => Self::UnsignedByte(unsafe { v.to_int_unchecked() }),
                Self::Number(v) => Self::UnsignedByte(unsafe { v.to_int_unchecked() }),
                Self::BigInt(v) => Self::UnsignedByte(v.try_into().unwrap_or(0)),
                Self::Byte(v) => Self::UnsignedByte((*v).try_into().unwrap_or(0)),
                Self::Short(v) => Self::UnsignedByte((*v).try_into().unwrap_or(0)),
                Self::Int(v) => Self::UnsignedByte((*v).try_into().unwrap_or(0)),
                Self::Long(v) => Self::UnsignedByte((*v).try_into().unwrap_or(0)),
                Self::UnsignedByte(v) => Self::UnsignedByte(*v),
                Self::UnsignedShort(v) => Self::UnsignedByte((*v).try_into().unwrap_or(0)),
                Self::UnsignedInt(v) => Self::UnsignedByte((*v).try_into().unwrap_or(0)),
                Self::UnsignedLong(v) => Self::UnsignedByte((*v).try_into().unwrap_or(0)),
            }
        } else if target_type == &unsigned_short_type {
            match self {
                Self::Single(v) => Self::UnsignedShort(unsafe { v.to_int_unchecked() }),
                Self::Number(v) => Self::UnsignedShort(unsafe { v.to_int_unchecked() }),
                Self::BigInt(v) => Self::UnsignedShort(v.try_into().unwrap_or(0)),
                Self::Byte(v) => Self::UnsignedShort((*v).try_into().unwrap_or(0)),
                Self::Short(v) => Self::UnsignedShort((*v).try_into().unwrap_or(0)),
                Self::Int(v) => Self::UnsignedShort((*v).try_into().unwrap_or(0)),
                Self::Long(v) => Self::UnsignedShort((*v).try_into().unwrap_or(0)),
                Self::UnsignedByte(v) => Self::UnsignedShort(*v as u16),
                Self::UnsignedShort(v) => Self::UnsignedShort(*v),
                Self::UnsignedInt(v) => Self::UnsignedShort((*v).try_into().unwrap_or(0)),
                Self::UnsignedLong(v) => Self::UnsignedShort((*v).try_into().unwrap_or(0)),
            }
        } else if target_type == &unsigned_int_type {
            match self {
                Self::Single(v) => Self::UnsignedInt(unsafe { v.to_int_unchecked() }),
                Self::Number(v) => Self::UnsignedInt(unsafe { v.to_int_unchecked() }),
                Self::BigInt(v) => Self::UnsignedInt(v.try_into().unwrap_or(0)),
                Self::Byte(v) => Self::UnsignedInt((*v).try_into().unwrap_or(0)),
                Self::Short(v) => Self::UnsignedInt((*v).try_into().unwrap_or(0)),
                Self::Int(v) => Self::UnsignedInt((*v).try_into().unwrap_or(0)),
                Self::Long(v) => Self::UnsignedInt((*v).try_into().unwrap_or(0)),
                Self::UnsignedByte(v) => Self::UnsignedInt(*v as u32),
                Self::UnsignedShort(v) => Self::UnsignedInt(*v as u32),
                Self::UnsignedInt(v) => Self::UnsignedInt((*v).try_into().unwrap_or(0)),
                Self::UnsignedLong(v) => Self::UnsignedInt((*v).try_into().unwrap_or(0)),
            }
        } else if target_type == &unsigned_long_type {
            match self {
                Self::Single(v) => Self::UnsignedLong(unsafe { v.to_int_unchecked() }),
                Self::Number(v) => Self::UnsignedLong(unsafe { v.to_int_unchecked() }),
                Self::BigInt(v) => Self::UnsignedLong(v.try_into().unwrap_or(0)),
                Self::Byte(v) => Self::UnsignedLong((*v).try_into().unwrap_or(0)),
                Self::Short(v) => Self::UnsignedLong((*v).try_into().unwrap_or(0)),
                Self::Int(v) => Self::UnsignedLong((*v).try_into().unwrap_or(0)),
                Self::Long(v) => Self::UnsignedLong((*v).try_into().unwrap_or(0)),
                Self::UnsignedByte(v) => Self::UnsignedLong(*v as u64),
                Self::UnsignedShort(v) => Self::UnsignedLong(*v as u64),
                Self::UnsignedInt(v) => Self::UnsignedLong(*v as u64),
                Self::UnsignedLong(v) => Self::UnsignedLong(*v),
            }
        } else if target_type == &big_int_type {
            match self {
                Self::Single(v) => Self::BigInt(BigInt::from_f32(*v).unwrap_or(BigInt::zero())),
                Self::Number(v) => Self::BigInt(BigInt::from_f64(*v).unwrap_or(BigInt::zero())),
                Self::BigInt(v) => Self::BigInt(v.clone()),
                Self::Byte(v) => Self::BigInt((*v).into()),
                Self::Short(v) => Self::BigInt((*v).into()),
                Self::Int(v) => Self::BigInt((*v).into()),
                Self::Long(v) => Self::BigInt((*v).into()),
                Self::UnsignedByte(v) => Self::BigInt((*v).into()),
                Self::UnsignedShort(v) => Self::BigInt((*v).into()),
                Self::UnsignedInt(v) => Self::BigInt((*v).into()),
                Self::UnsignedLong(v) => Self::BigInt((*v).into()),
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