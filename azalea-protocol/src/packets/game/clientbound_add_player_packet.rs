use packet_macros::{GamePacket, McBuf};
use uuid::Uuid;

/// This packet is sent by the server when a player comes into visible range, not when a player joins.
#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundAddPlayerPacket {
    #[var]
    pub id: u32,
    pub uuid: Uuid,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_rot: i8,
    pub y_rot: i8,
}
