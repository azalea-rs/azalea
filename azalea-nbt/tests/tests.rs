use azalea_nbt::Tag;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::test]
async fn test_decode_hello_world() {
    // read hello_world.nbt
    let mut file = File::open("tests/hello_world.nbt").await.unwrap();
    let tag = Tag::read(&mut file).await.unwrap();
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

#[tokio::test]
async fn test_roundtrip_hello_world() {
    let mut file = File::open("tests/hello_world.nbt").await.unwrap();
    let mut original = Vec::new();
    file.read_to_end(&mut original).await.unwrap();

    let mut original_stream = Cursor::new(original.clone());
    let tag = Tag::read(&mut original_stream).await.unwrap();

    // write hello_world.nbt
    let mut result = Cursor::new(Vec::new());
    tag.write(&mut result).unwrap();

    assert_eq!(result.into_inner(), original);
}

#[tokio::test]
async fn test_bigtest() {
    // read bigtest.nbt
    let mut file = File::open("tests/bigtest.nbt").await.unwrap();
    let mut original = Vec::new();
    file.read_to_end(&mut original).await.unwrap();

    let mut original_stream = Cursor::new(original.clone());
    let original_tag = Tag::read_gzip(&mut original_stream).await.unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result).unwrap();

    let decoded_tag = Tag::read(&mut Cursor::new(result)).await.unwrap();

    assert_eq!(decoded_tag, original_tag);
}

#[tokio::test]
async fn test_stringtest() {
    let correct_tag = Tag::Compound(HashMap::from_iter(vec![(
        "😃".to_string(),
        Tag::List(vec![
            Tag::String("asdfkghasfjgihsdfogjsndfg".to_string()),
            Tag::String("jnabsfdgihsabguiqwrntgretqwejirhbiqw".to_string()),
            Tag::String("asd".to_string()),
            Tag::String("wqierjgt7wqy8u4rtbwreithwretiwerutbwenryq8uwervqwer9iuqwbrgyuqrbtwierotugqewrtqwropethert".to_string()),
            Tag::String("asdf".to_string()),
            Tag::String("alsdkjiqwoe".to_string()),
            Tag::String("lmqi9hyqd".to_string()),
            Tag::String("qwertyuiop".to_string()),
            Tag::String("asdfghjkl".to_string()),
            Tag::String("zxcvbnm".to_string()),
            Tag::String("                               ".to_string()),
            Tag::String("words words words words words words".to_string()),
            Tag::String("aaaaaaaaaaaaaaaaaaaa".to_string()),
            Tag::String("♥".to_string()),
            Tag::String("a\nb\n\n\nc\r\rd".to_string()),
            Tag::String("😁".to_string()),
        ])
    )]));
    let mut file = std::fs::File::open("tests/stringtest.nbt").unwrap();
    let mut original = Vec::new();
    file.read_to_end(&mut original).unwrap();

    let mut original_stream = Cursor::new(original.clone());
    let original_tag = Tag::read_gzip(&mut original_stream).await.unwrap();

    assert_eq!(original_tag, correct_tag);
}

#[tokio::test]
async fn test_complex_player() {
    let mut file = File::open("tests/complex_player.dat").await.unwrap();
    let mut original = Vec::new();
    file.read_to_end(&mut original).await.unwrap();

    let mut original_stream = Cursor::new(original.clone());
    let original_tag = Tag::read_gzip(&mut original_stream).await.unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result).unwrap();

    let decoded_tag = Tag::read(&mut Cursor::new(result)).await.unwrap();

    assert_eq!(decoded_tag, original_tag);
}

#[tokio::test]
async fn test_simple_player() {
    let mut file = File::open("tests/simple_player.dat").await.unwrap();
    let mut original = Vec::new();
    file.read_to_end(&mut original).await.unwrap();

    let mut original_stream = Cursor::new(original.clone());
    let original_tag = Tag::read_gzip(&mut original_stream).await.unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result).unwrap();

    let decoded_tag = Tag::read(&mut Cursor::new(result)).await.unwrap();

    assert_eq!(decoded_tag, original_tag);
}
