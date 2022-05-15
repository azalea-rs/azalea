use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundKeepAlivePacket {
    pub id: u64,
}
