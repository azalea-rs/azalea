use azalea_nbt::Tag;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

#[test]
fn test_decode_hello_world() {
    // read hello_world.nbt
    let mut file = std::fs::File::open("tests/hello_world.nbt").unwrap();
    let tag = Tag::read(&mut file).unwrap();
    assert_eq!(
        tag,
        Tag::Compound(HashMap::from_iter(vec![(
            "hello world".to_string(),
            Tag::Compound(HashMap::from_iter(vec![(
                "name".to_string(),
                Tag::String("Bananrama".to_string()),
            )]))
        )]))
    );
}

#[test]
fn test_roundtrip_hello_world() {
    let mut file = std::fs::File::open("tests/hello_world.nbt").unwrap();
    let mut original = Vec::new();
    file.read_to_end(&mut original).unwrap();

    let mut original_stream = Cursor::new(original.clone());
    let tag = Tag::read(&mut original_stream).unwrap();

    println!("ok read {:?}", tag);

    // write hello_world.nbt
    let mut result = Cursor::new(Vec::new());
    tag.write(&mut result).unwrap();

    assert_eq!(result.into_inner(), original);
}

#[test]
fn test_bigtest() {
    // read bigtest.nbt
    let mut file = std::fs::File::open("tests/bigtest.nbt").unwrap();
    let mut original = Vec::new();
    file.read_to_end(&mut original).unwrap();

    let mut original_stream = Cursor::new(original.clone());
    let original_tag = Tag::read_gzip(&mut original_stream).unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result).unwrap();

    let decoded_tag = Tag::read(&mut Cursor::new(result)).unwrap();

    assert_eq!(decoded_tag, original_tag);
}
