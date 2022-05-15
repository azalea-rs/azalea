use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ServerboundKeepAlivePacket {
    pub id: u64,
}
