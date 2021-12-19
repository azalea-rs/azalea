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
}
