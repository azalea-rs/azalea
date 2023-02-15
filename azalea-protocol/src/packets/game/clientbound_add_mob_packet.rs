use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundAddMobPacket {
    #[var]
    pub id: u32,
    pub uuid: Uuid,
    #[var]
    pub kind: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub y_rot: i8,
    pub x_rot: i8,
    pub y_head_rot: i8,
    pub xd: u16,
    pub yd: u16,
    pub zd: u16,
}
