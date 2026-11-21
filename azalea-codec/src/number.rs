use crate::{
    Codec, Ops,
    error::{ReadError, WriteError},
};

// a port of Java's Number abstract class
#[derive(Copy, Clone, Debug)]
pub enum Number {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}

impl Number {
    /// Whether the number is an integer, i.e. not a float or double.
    pub fn is_numeric_integer(&self) -> bool {
        !matches!(self, Self::Float(_) | Self::Double(_))
    }
}

impl<O: Ops> Codec<O> for Number {
    fn codec_read(ops: &O) -> crate::error::ReadResult<Self> {
        ops.get_number()
    }
    fn codec_write(&self) -> crate::error::WriteResult<O> {
        O::create_number(*self)
    }
}

impl TryFrom<Number> for i64 {
    type Error = ReadError;
    fn try_from(n: Number) -> Result<Self, Self::Error> {
        Ok(match n {
            Number::Byte(v) => v as i64,
            Number::Short(v) => v as i64,
            Number::Int(v) => v as i64,
            Number::Long(v) => v as i64,
            Number::Float(_) | Number::Double(_) => return Err(ReadError::number_too_big(n)),
        })
    }
}

impl From<Number> for f64 {
    fn from(n: Number) -> Self {
        match n {
            Number::Byte(v) => v as f64,
            Number::Short(v) => v as f64,
            Number::Int(v) => v as f64,
            Number::Long(v) => v as f64,
            Number::Float(v) => v as f64,
            Number::Double(v) => v as f64,
        }
    }
}
impl From<Number> for f32 {
    fn from(n: Number) -> Self {
        f64::from(n) as f32
    }
}

macro_rules! try_from_number {
    ($($n:ty),*) => {
        $(
            impl TryFrom<Number> for $n {
                type Error = ReadError;
                fn try_from(n: Number) -> Result<Self, Self::Error> {
                    i64::try_from(n)
                        .and_then(|v| v.try_into().map_err(|_| ReadError::number_too_big(n)))
                }
            }
        )*
    };
}
try_from_number!(
    i8, i16, i32, i128, //
    u8, u16, u32, u64
);

macro_rules! from_literal {
    ($n:ty, $variant:ident) => {
        impl From<$n> for Number {
            fn from(n: $n) -> Self {
                Self::$variant(n)
            }
        }
    };
}
from_literal!(i64, Long);
from_literal!(i32, Int);
from_literal!(i16, Short);
from_literal!(i8, Byte);
from_literal!(f32, Float);
from_literal!(f64, Double);

impl From<u8> for Number {
    fn from(n: u8) -> Self {
        Self::Byte(n as i8)
    }
}

impl From<u32> for Number {
    fn from(n: u32) -> Self {
        Self::Long(n as i64)
    }
}
impl From<u16> for Number {
    fn from(n: u16) -> Self {
        Self::Int(n as i32)
    }
}

impl TryFrom<u64> for Number {
    type Error = WriteError;
    fn try_from(n: u64) -> Result<Self, Self::Error> {
        Ok(Self::Long(
            n.try_into().map_err(|_| WriteError::number_too_big(n))?,
        ))
    }
}
