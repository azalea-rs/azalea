use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetTimePacket {
    pub game_time: u64,
    pub day_time: u64,
}
