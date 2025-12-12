# `azalea-registry`

Minecraft's registries are a system that give identifiers to certain enums the game.

Some registries, defined in [`crate::builtin`], are static for the client and server. This includes blocks and items.

Other registries, defined in [`crate::data`], are sent to us by the server. This includes things such as enchantments and biomes.

