# Azalea NBT

A fast NBT serializer and deserializer.

# Examples

```
use ahash::AHashMap;
use azalea_nbt::Tag;
use std::{io::{Cursor, Read}, fs::File};

let mut file = File::open("tests/hello_world.nbt").unwrap();
let mut buf = vec![];
file.read_to_end(&mut buf).unwrap();
let tag = Tag::read(&mut Cursor::new(&buf[..])).unwrap();
assert_eq!(
    tag,
    Tag::Compound(AHashMap::from_iter(vec![(
        "hello world".to_string(),
        Tag::Compound(AHashMap::from_iter(vec![(
            "name".to_string(),
            Tag::String("Bananrama".to_string()),
        )]))
    )]))
);
```
