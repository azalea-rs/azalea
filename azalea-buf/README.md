# Azalea Buf

An implementation of Minecraft's FriendlyByteBuf. This is used frequently in the game for serialization and deserialization of data.

Note that there are some minor implementation differences such as using unsigned integers in places where Minecraft uses signed integers. This doesn't cause issues normally, but does technically make usage of azalea-buf detectable if a server really wants to since it won't error in  places where vanilla Minecraft would.
