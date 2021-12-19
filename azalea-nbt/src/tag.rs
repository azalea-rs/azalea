use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Tag {
    End,                            // 0
    Byte(i8),                       // 1
    Short(i16),                     // 2
    Int(i32),                       // 3
    Long(i64),                      // 4
    Float(f32),                     // 5
    Double(f64),                    // 6
    ByteArray(Vec<i8>),             // 7
    String(String),                 // 8
    List(Vec<Tag>),                 // 9
    Compound(HashMap<String, Tag>), // 10
    IntArray(Vec<i32>),             // 11
    LongArray(Vec<i64>),            // 12
}

impl Tag {
    pub fn id(&self) -> u8 {
        match self {
            Tag::End => 0,
            Tag::Byte(value) => 1,
            Tag::Short(value) => 2,
            Tag::Int(value) => 3,
            Tag::Long(value) => 4,
            Tag::Float(value) => 5,
            Tag::Double(value) => 6,
            Tag::ByteArray(value) => 7,
            Tag::String(value) => 8,
            Tag::List(value) => 9,
            Tag::Compound(value) => 10,
            Tag::IntArray(value) => 11,
            Tag::LongArray(value) => 12,
        }
    }
}
