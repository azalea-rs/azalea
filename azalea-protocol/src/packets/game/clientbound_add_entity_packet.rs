use packet_macros::{GamePacket, McBuf};
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundAddEntityPacket {
    #[var]
    pub id: i32,
    pub uuid: Uuid,
    // TODO: have an entity type struct
    #[var]
    pub entity_type: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_rot: i8,
    pub y_rot: i8,
    // pub y_head_rot: i8,
    pub data: i32,
    pub x_vel: u16,
    pub y_vel: u16,
    pub z_vel: u16,
}
