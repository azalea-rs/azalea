use azalea_buf::McBuf;
use azalea_core::PositionDelta8;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundMoveEntityPosPacket {
    #[var]
    pub entity_id: u32,
    pub delta: PositionDelta8,
    pub on_ground: bool,
}
