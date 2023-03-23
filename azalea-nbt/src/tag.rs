use ahash::AHashMap;

use compact_str::CompactString;
use enum_as_inner::EnumAsInner;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type NbtByte = i8;
pub type NbtShort = i16;
pub type NbtInt = i32;
pub type NbtLong = i64;
pub type NbtFloat = f32;
pub type NbtDouble = f64;
pub type NbtByteArray = Vec<u8>;
pub type NbtString = CompactString;
pub type NbtCompound = AHashMap<CompactString, Tag>;
pub type NbtIntArray = Vec<i32>;
pub type NbtLongArray = Vec<i64>;

pub const END_ID: u8 = 0;
pub const BYTE_ID: u8 = 1;
pub const SHORT_ID: u8 = 2;
pub const INT_ID: u8 = 3;
pub const LONG_ID: u8 = 4;
pub const FLOAT_ID: u8 = 5;
pub const DOUBLE_ID: u8 = 6;
pub const BYTE_ARRAY_ID: u8 = 7;
pub const STRING_ID: u8 = 8;
pub const LIST_ID: u8 = 9;
pub const COMPOUND_ID: u8 = 10;
pub const INT_ARRAY_ID: u8 = 11;
pub const LONG_ARRAY_ID: u8 = 12;

/// An NBT value.
#[derive(Clone, Debug, PartialEq, Default, EnumAsInner)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum Tag {
    #[default]
    End = END_ID,
    Byte(NbtByte) = BYTE_ID,
    Short(NbtShort) = SHORT_ID,
    Int(NbtInt) = INT_ID,
    Long(NbtLong) = LONG_ID,
    Float(NbtFloat) = FLOAT_ID,
    Double(NbtDouble) = DOUBLE_ID,
    ByteArray(NbtByteArray) = BYTE_ARRAY_ID,
    String(NbtString) = STRING_ID,
    List(NbtList) = LIST_ID,
    Compound(NbtCompound) = COMPOUND_ID,
    IntArray(NbtIntArray) = INT_ARRAY_ID,
    LongArray(NbtLongArray) = LONG_ARRAY_ID,
}

/// An NBT value.
#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum NbtList {
    Empty = END_ID,
    Byte(Vec<NbtByte>) = BYTE_ID,
    Short(Vec<NbtShort>) = SHORT_ID,
    Int(Vec<NbtInt>) = INT_ID,
    Long(Vec<NbtLong>) = LONG_ID,
    Float(Vec<NbtFloat>) = FLOAT_ID,
    Double(Vec<NbtDouble>) = DOUBLE_ID,
    ByteArray(Vec<NbtByteArray>) = BYTE_ARRAY_ID,
    String(Vec<NbtString>) = STRING_ID,
    List(Vec<NbtList>) = LIST_ID,
    Compound(Vec<NbtCompound>) = COMPOUND_ID,
    IntArray(Vec<NbtIntArray>) = INT_ARRAY_ID,
    LongArray(Vec<NbtLongArray>) = LONG_ARRAY_ID,
}

impl Tag {
    /// Get the numerical ID of the tag type.
    #[inline]
    pub fn id(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)`
        // `union` between `repr(C)` structs, each of which has the `u8`
        // discriminant as its first field, so we can read the discriminant
        // without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}
impl NbtList {
    /// Get the numerical ID of the tag type.
    #[inline]
    pub fn id(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)`
        // `union` between `repr(C)` structs, each of which has the `u8`
        // discriminant as its first field, so we can read the discriminant
        // without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}
