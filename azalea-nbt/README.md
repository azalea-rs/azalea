# Azalea NBT

A fast NBT serializer and deserializer.

# Examples

```
use ahash::AHashMap;
use azalea_nbt::Tag;
use std::io::Cursor;

let buf = include_bytes!("../tests/hello_world.nbt");
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
```
