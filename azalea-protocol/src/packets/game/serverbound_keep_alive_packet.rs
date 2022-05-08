use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ServerboundKeepAlivePacket {
    pub id: u64,
}
