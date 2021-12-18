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
