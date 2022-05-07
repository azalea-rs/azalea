use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundSetTimePacket {
    pub game_time: u64,
    pub day_time: u64,
}
