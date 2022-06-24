use azalea_buf::McBuf;
use azalea_core::PositionDelta;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundMoveEntityPosPacket {
    #[var]
    pub entity_id: u32,
    pub delta: PositionDelta,
    pub on_ground: bool,
}
