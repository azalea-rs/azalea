# `azalea-codec`

A port of [Mojang's serialization library](https://github.com/Mojang/DataFixerUpper/tree/master/src/main/java/com/mojang/serialization) but with really cool macros.

The library support NBT (when the `simdnbt` feature is enabled) and JSON (when the `serde_json` feature is enabled), but more serialization formats can be added by the user.
