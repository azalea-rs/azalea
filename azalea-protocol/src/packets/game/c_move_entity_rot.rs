use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundMoveEntityRot {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub y_rot: i8,
    pub x_rot: i8,
    pub on_ground: bool,
}
