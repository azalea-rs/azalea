use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundEntityVelocityPacket {
    #[varint]
    pub entity_id: u32,
    pub x_vel: i16,
    pub y_vel: i16,
    pub z_vel: i16,
}
