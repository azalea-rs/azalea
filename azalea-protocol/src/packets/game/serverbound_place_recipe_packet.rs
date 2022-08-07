use azalea_buf::McBuf;
use azalea_core::ResourceLocation;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundPlaceRecipePacket {
    pub container_id: u8,
    pub recipe: ResourceLocation,
    pub shift_down: bool,
}
