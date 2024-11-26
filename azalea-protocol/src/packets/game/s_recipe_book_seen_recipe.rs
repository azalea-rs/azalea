use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundRecipeBookSeenRecipe {
    pub recipe: ResourceLocation,
}
