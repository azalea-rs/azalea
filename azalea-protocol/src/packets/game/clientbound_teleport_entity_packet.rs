use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundTeleportEntityPacket {
    #[var]
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub y_rot: i8,
    pub x_rot: i8,
    pub on_ground: bool,
}
