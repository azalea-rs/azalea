use azalea_buf::McBuf;
use azalea_core::ResourceLocation;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundRecipeBookSeenRecipePacket {
    pub recipe: ResourceLocation,
}
