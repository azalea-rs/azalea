use ahash::AHashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tag {
    End,                             // 0
    Byte(i8),                        // 1
    Short(i16),                      // 2
    Int(i32),                        // 3
    Long(i64),                       // 4
    Float(f32),                      // 5
    Double(f64),                     // 6
    ByteArray(Vec<u8>),              // 7
    String(String),                  // 8
    List(Vec<Tag>),                  // 9
    Compound(AHashMap<String, Tag>), // 10
    IntArray(Vec<i32>),              // 11
    LongArray(Vec<i64>),             // 12
}

impl Default for Tag {
    fn default() -> Self {
        Tag::End
    }
}

impl Tag {
    #[inline]
    pub fn id(&self) -> u8 {
        match self {
            Tag::End => 0,
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) => 3,
            Tag::Long(_) => 4,
            Tag::Float(_) => 5,
            Tag::Double(_) => 6,
            Tag::ByteArray(_) => 7,
            Tag::String(_) => 8,
            Tag::List(_) => 9,
            Tag::Compound(_) => 10,
            Tag::IntArray(_) => 11,
            Tag::LongArray(_) => 12,
        }
    }

    #[inline]
    pub fn as_byte(&self) -> Option<&i8> {
        if let Tag::Byte(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_short(&self) -> Option<&i16> {
        if let Tag::Short(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_int(&self) -> Option<&i32> {
        if let Tag::Int(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_long(&self) -> Option<&i64> {
        if let Tag::Long(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_float(&self) -> Option<&f32> {
        if let Tag::Float(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_double(&self) -> Option<&f64> {
        if let Tag::Double(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_string(&self) -> Option<&str> {
        if let Tag::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_compound(&self) -> Option<&AHashMap<String, Tag>> {
        if let Tag::Compound(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_bytearray(&self) -> Option<&[u8]> {
        if let Tag::ByteArray(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_intarray(&self) -> Option<&Vec<i32>> {
        if let Tag::IntArray(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_longarray(&self) -> Option<&Vec<i64>> {
        if let Tag::LongArray(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_list(&self) -> Option<&[Tag]> {
        if let Tag::List(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
