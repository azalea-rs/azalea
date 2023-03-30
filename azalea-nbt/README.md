# Azalea NBT

A fast NBT serializer and deserializer.

- Gzip and Zlib compression
- All data is owned for ease-of-use
- Serde support with the `serde` feature.

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

# Benchmarks

At the time of writing, Azalea NBT is the fastest NBT decoder (approximately twice as fast as Graphite NBT, the second fastest) and on-par with the fastest NBT encoders (sometimes the fastest, depending on the data).

You can run the benchmarks to compare against other NBT libraries with `cargo bench --bench compare` and the normal benchmarks with `cargo bench --bench nbt`.

Note: For best performance, use `RUSTFLAGS='-C target-cpu=native'`.
