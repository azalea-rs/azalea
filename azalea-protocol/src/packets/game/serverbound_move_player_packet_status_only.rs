use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ServerboundMovePlayerPacketStatusOnly {
    pub on_ground: bool,
}
