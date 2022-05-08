use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundKeepAlivePacket {
    pub id: u64,
}
