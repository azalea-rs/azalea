# Azalea NBT

A fast NBT serializer and deserializer.

# Examples

```
use azalea_nbt::{Tag, NbtCompound};
use std::io::Cursor;

let buf = include_bytes!("../tests/hello_world.nbt");
let tag = Tag::read(&mut Cursor::new(&buf[..])).unwrap();
assert_eq!(
    tag,
    Tag::Compound(NbtCompound::from_iter(vec![(
        "hello world".into(),
        Tag::Compound(NbtCompound::from_iter(vec![(
            "name".into(),
            Tag::String("Bananrama".into()),
        )]))
    )]))
);
```
