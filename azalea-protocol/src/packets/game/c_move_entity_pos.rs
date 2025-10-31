use azalea_buf::AzBuf;
use azalea_core::delta::PositionDelta8;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ClientboundGamePacket)]
pub struct ClientboundMoveEntityPos {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub delta: PositionDelta8,
    pub on_ground: bool,
}
