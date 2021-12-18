use azalea_nbt::Tag;
use std::collections::HashMap;

#[test]
fn test_hello_world() {
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
