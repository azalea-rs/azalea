use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetBorderCenter {
    pub new_center_x: f64,
    pub new_center_z: f64,
}
