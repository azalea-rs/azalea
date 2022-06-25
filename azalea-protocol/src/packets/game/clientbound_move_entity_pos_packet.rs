use azalea_buf::McBuf;
use azalea_core::PositionDelta8;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundMoveEntityPosPacket {
    #[var]
    pub entity_id: u32,
    pub delta: PositionDelta8,
    pub on_ground: bool,
}
