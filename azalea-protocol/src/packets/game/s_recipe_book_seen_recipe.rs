use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ServerboundGamePacket)]
pub struct ServerboundRecipeBookSeenRecipe {
    pub recipe: ResourceLocation,
}
