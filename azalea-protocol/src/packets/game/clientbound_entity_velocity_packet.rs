use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundEntityVelocityPacket {
    #[var]
    pub entity_id: u32,
    pub x_vel: i16,
    pub y_vel: i16,
    pub z_vel: i16,
}
