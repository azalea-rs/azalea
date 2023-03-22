use ahash::AHashMap;
use azalea_nbt::Tag;
use std::io::Cursor;

#[test]
fn test_decode_hello_world() {
    // read hello_world.nbt
    let buf = include_bytes!("hello_world.nbt").to_vec();
    let tag = Tag::read(&mut Cursor::new(&buf[..])).unwrap();
    assert_eq!(
        tag,
        Tag::Compound(AHashMap::from_iter(vec![(
            "hello world".into(),
            Tag::Compound(AHashMap::from_iter(vec![(
                "name".into(),
                Tag::String("Bananrama".into()),
            )]))
        )]))
    );
}

#[test]
fn test_roundtrip_hello_world() {
    let original = include_bytes!("hello_world.nbt").to_vec();

    let mut original_stream = Cursor::new(&original[..]);
    let tag = Tag::read(&mut original_stream).unwrap();

    // write hello_world.nbt
    let mut result = Vec::new();
    tag.write(&mut result).unwrap();

    assert_eq!(result, original);
}

#[test]
fn test_bigtest() {
    // read bigtest.nbt
    let original = include_bytes!("bigtest.nbt").to_vec();

    let mut original_stream = Cursor::new(original);
    let original_tag = Tag::read_gzip(&mut original_stream).unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result).unwrap();

    let decoded_tag = Tag::read(&mut Cursor::new(&result)).unwrap();

    assert_eq!(decoded_tag, original_tag);
}

#[test]
fn test_stringtest() {
    let correct_tag = Tag::Compound(AHashMap::from_iter(vec![(
        "😃".into(),
        Tag::List(vec![
            Tag::String("asdfkghasfjgihsdfogjsndfg".into()),
            Tag::String("jnabsfdgihsabguiqwrntgretqwejirhbiqw".into()),
            Tag::String("asd".into()),
            Tag::String("wqierjgt7wqy8u4rtbwreithwretiwerutbwenryq8uwervqwer9iuqwbrgyuqrbtwierotugqewrtqwropethert".into()),
            Tag::String("asdf".into()),
            Tag::String("alsdkjiqwoe".into()),
            Tag::String("lmqi9hyqd".into()),
            Tag::String("qwertyuiop".into()),
            Tag::String("asdfghjkl".into()),
            Tag::String("zxcvbnm".into()),
            Tag::String("                               ".into()),
            Tag::String("words words words words words words".into()),
            Tag::String("aaaaaaaaaaaaaaaaaaaa".into()),
            Tag::String("♥".into()),
            Tag::String("a\nb\n\n\nc\r\rd".into()),
            Tag::String("😁".into()),
        ])
    )]));
    let original = include_bytes!("stringtest.nbt").to_vec();

    let mut original_stream = Cursor::new(original);
    let original_tag = Tag::read_gzip(&mut original_stream).unwrap();

    assert_eq!(original_tag, correct_tag);
}

#[test]
fn test_complex_player() {
    let original = include_bytes!("complex_player.dat").to_vec();

    let mut original_stream = Cursor::new(original);
    let original_tag = Tag::read_gzip(&mut original_stream).unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result).unwrap();

    let decoded_tag = Tag::read(&mut Cursor::new(&result)).unwrap();

    assert_eq!(decoded_tag, original_tag);
}

#[test]
fn test_simple_player() {
    let original = include_bytes!("simple_player.dat").to_vec();

    let mut original_stream = Cursor::new(original);
    let original_tag = Tag::read_gzip(&mut original_stream).unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result).unwrap();

    let decoded_tag = Tag::read(&mut Cursor::new(&result)).unwrap();

    assert_eq!(decoded_tag, original_tag);
}
