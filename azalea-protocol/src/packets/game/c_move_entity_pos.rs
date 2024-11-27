use azalea_buf::AzBuf;
use azalea_core::delta::PositionDelta8;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundMoveEntityPos {
    #[var]
    pub entity_id: u32,
    pub delta: PositionDelta8,
    pub on_ground: bool,
}
