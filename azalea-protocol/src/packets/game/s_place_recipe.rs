use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundPlaceRecipe {
    pub container_id: u8,
    pub recipe: ResourceLocation,
    pub shift_down: bool,
}
