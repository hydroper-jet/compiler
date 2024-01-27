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
}