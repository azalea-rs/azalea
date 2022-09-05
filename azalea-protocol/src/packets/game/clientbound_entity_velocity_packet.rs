use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundEntityVelocityPacket {
    #[var]
    pub entity_id: u32,
    pub x_vel: i16,
    pub y_vel: i16,
    pub z_vel: i16,
}
