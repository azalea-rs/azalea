use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundBlockChangedAckPacket {
    #[var]
    pub sequence: i32,
}
