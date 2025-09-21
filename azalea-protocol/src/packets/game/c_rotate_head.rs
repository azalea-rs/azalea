use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundRotateHead {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub y_head_rot: i8,
}
