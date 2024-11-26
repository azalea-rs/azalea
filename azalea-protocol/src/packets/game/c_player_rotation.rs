use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerRotation {
    pub y_rot: f32,
    pub x_rot: f32,
}
