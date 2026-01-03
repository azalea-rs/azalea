use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundMoveEntityRot {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub y_rot: i8,
    pub x_rot: i8,
    pub on_ground: bool,
}
