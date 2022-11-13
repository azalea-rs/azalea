use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetBorderCenterPacket {
    pub new_center_x: f64,
    pub new_center_z: f64,
}
