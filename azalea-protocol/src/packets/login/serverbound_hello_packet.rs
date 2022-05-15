use packet_macros::{LoginPacket, McBuf};
use std::hash::Hash;

#[derive(Hash, Clone, Debug, McBuf, LoginPacket)]
pub struct ServerboundHelloPacket {
    pub username: String,
}
