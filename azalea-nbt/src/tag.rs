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

/// An NBT value.
#[derive(Clone, Debug, PartialEq, Default, EnumAsInner)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum Tag {
    #[default]
    End = 0,
    Byte(NbtByte) = 1,
    Short(NbtShort) = 2,
    Int(NbtInt) = 3,
    Long(NbtLong) = 4,
    Float(NbtFloat) = 5,
    Double(NbtDouble) = 6,
    ByteArray(NbtByteArray) = 7,
    String(NbtString) = 8,
    List(NbtList) = 9,
    Compound(NbtCompound) = 10,
    IntArray(NbtIntArray) = 11,
    LongArray(NbtLongArray) = 12,
}

/// An NBT value.
#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum NbtList {
    Empty,
    Byte(Vec<NbtByte>) = 1,
    Short(Vec<NbtShort>) = 2,
    Int(Vec<NbtInt>) = 3,
    Long(Vec<NbtLong>) = 4,
    Float(Vec<NbtFloat>) = 5,
    Double(Vec<NbtDouble>) = 6,
    ByteArray(Vec<NbtByteArray>) = 7,
    String(Vec<NbtString>) = 8,
    List(Vec<NbtList>) = 9,
    Compound(Vec<NbtCompound>) = 10,
    IntArray(Vec<NbtIntArray>) = 11,
    LongArray(Vec<NbtLongArray>) = 12,
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
