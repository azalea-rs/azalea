use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundMoveEntityPosPacket {
    #[var]
    pub entity_id: i32,
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
    pub on_ground: bool,
}
