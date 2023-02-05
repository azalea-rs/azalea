use azalea_buf::McBuf;
use azalea_core::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundTeleportEntityPacket {
    #[var]
    pub id: u32,
    pub position: Vec3,
    pub y_rot: i8,
    pub x_rot: i8,
    pub on_ground: bool,
}
