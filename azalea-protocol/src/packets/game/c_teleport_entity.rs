use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundTeleportEntity {
    #[var]
    pub id: u32,
    pub position: Vec3,
    pub y_rot: i8,
    pub x_rot: i8,
    pub on_ground: bool,
}
