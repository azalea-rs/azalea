use packet_macros::LoginPacket;
use std::hash::Hash;

#[derive(Hash, Clone, Debug, LoginPacket)]
pub struct ServerboundHelloPacket {
    pub username: String,
}
