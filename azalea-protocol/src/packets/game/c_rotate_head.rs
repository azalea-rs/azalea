use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundRotateHead {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub y_head_rot: i8,
}
