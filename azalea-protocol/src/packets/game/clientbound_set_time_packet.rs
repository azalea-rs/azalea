use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetTimePacket {
    pub game_time: u64,
    pub day_time: u64,
}
