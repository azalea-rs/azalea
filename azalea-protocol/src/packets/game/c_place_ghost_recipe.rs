use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundPlaceGhostRecipe {
    pub container_id: u8,
    pub recipe: ResourceLocation,
}
