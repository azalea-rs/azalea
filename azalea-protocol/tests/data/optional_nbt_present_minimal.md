# optional_nbt_present_minimal.bin

Minimal present `OptionalNbt` fixture for the unnamed compound `{a: 1b}`.

Bytes:

```text
07 0a 01 00 01 61 01 00
```

Layout:

- `07`: VarInt byte length of the unnamed NBT payload.
- `0a`: root compound tag.
- `01 00 01 61 01`: byte tag named `a` with value `1`.
- `00`: end of compound.
