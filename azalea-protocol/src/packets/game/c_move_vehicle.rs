use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundMoveVehicle {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub y_rot: f32,
    pub x_rot: f32,
}
