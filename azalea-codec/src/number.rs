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

macro_rules! number_from_other_impl {
    ($from_ty:ty, $stored_as_ty:ty, $intermediate_ty:ty, $variant:ident) => {
        impl From<$from_ty> for Number {
            fn from(n: $from_ty) -> Self {
                Self::$variant(n as $intermediate_ty as $stored_as_ty)
            }
        }
        impl From<Number> for $from_ty {
            fn from(n: Number) -> Self {
                <$intermediate_ty>::from(n) as $from_ty
            }
        }
    };
}
number_from_other_impl!(i8, i8, i32, Byte);
number_from_other_impl!(i16, i16, i32, Short);
number_from_other_impl!(u8, i8, i8, Byte);

macro_rules! number_cast_impl {
    ($t:ty, $variant:ident) => {
        impl From<$t> for Number {
            fn from(n: $t) -> Self {
                Self::$variant(n)
            }
        }
        impl From<Number> for $t {
            fn from(n: Number) -> Self {
                cast_to!(n, $t)
            }
        }
    };
}

// truncating and rounding is intentional, per https://docs.oracle.com/javase/8/docs/api/java/lang/Number.html
macro_rules! cast_to {
    ($i:ident, $t:ty) => {
        match $i {
            Number::Byte(v) => v as $t,
            Number::Short(v) => v as $t,
            Number::Int(v) => v as $t,
            Number::Long(v) => v as $t,
            Number::Float(v) => v as $t,
            Number::Double(v) => v as $t,
        }
    };
}
number_cast_impl!(i32, Int);
number_cast_impl!(i64, Long);
number_cast_impl!(f32, Float);
number_cast_impl!(f64, Double);

impl From<Number> for u128 {
    fn from(n: Number) -> Self {
        cast_to!(n, u128)
    }
}
