use packet_macros::GamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundAddPlayerPacket {
    #[var]
    pub id: i32,
    pub uuid: Uuid,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_rot: i8,
    pub y_rot: i8,
}
