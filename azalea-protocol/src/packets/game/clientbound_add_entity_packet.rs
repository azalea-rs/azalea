use uuid::Uuid;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundAddEntityPacket {
#[var]
pub id: u32,
pub uuid: Uuid,
pub x: f64,
pub y: f64,
pub z: f64,
#[var]
pub x_rot: i32,
#[var]
pub y_rot: i32,
#[var]
pub y_head_rot: i32,
#[var]
pub data: u32,
#[var]
pub xa: i32,
#[var]
pub ya: i32,
#[var]
pub za: i32,
}