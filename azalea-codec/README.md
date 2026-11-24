# `azalea-codec`

A port of [Mojang's serialization library](https://github.com/Mojang/DataFixerUpper/tree/master/src/main/java/com/mojang/serialization) but with really cool macros.

The library comes with optional support for NBT (through `simdnbt`) and JSON (through `serde_json` or `simd-json`). More serialization formats can be implemented through the [`Ops`] trait.
