# Azalea NBT

A fast NBT serializer and deserializer.

# Examples

```
use azalea_nbt::{Nbt, NbtCompound};
use std::io::Cursor;

let buf = include_bytes!("../tests/hello_world.nbt");
let tag = Nbt::read(&mut Cursor::new(&buf[..])).unwrap();
assert_eq!(
    tag,
    Nbt::Compound(NbtCompound::from_iter(vec![(
        "hello world".into(),
        Nbt::Compound(NbtCompound::from_iter(vec![(
            "name".into(),
            Nbt::String("Bananrama".into()),
        )]))
    )]))
);
```
