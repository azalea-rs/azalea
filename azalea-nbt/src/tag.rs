use ahash::AHashMap;

use compact_str::CompactString;
use enum_as_inner::EnumAsInner;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An NBT value.
#[derive(Clone, Debug, PartialEq, Default, EnumAsInner)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum Tag {
    #[default]
    End = 0,
    Byte(i8) = 1,
    Short(i16) = 2,
    Int(i32) = 3,
    Long(i64) = 4,
    Float(f32) = 5,
    Double(f64) = 6,
    ByteArray(Vec<u8>) = 7,
    String(CompactString) = 8,
    List(Vec<Tag>) = 9,
    Compound(AHashMap<CompactString, Tag>) = 10,
    IntArray(Vec<i32>) = 11,
    LongArray(Vec<i64>) = 12,
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
