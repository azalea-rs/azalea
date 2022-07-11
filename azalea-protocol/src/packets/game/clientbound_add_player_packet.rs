use azalea_buf::McBuf;
use azalea_core::Vec3;
use azalea_world::entity::Entity;
use packet_macros::GamePacket;
use uuid::Uuid;

/// This packet is sent by the server when a player comes into visible range, not when a player joins.
#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundAddPlayerPacket {
    #[var]
    pub id: u32,
    pub uuid: Uuid,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_rot: i8,
    pub y_rot: i8,
}

impl From<&ClientboundAddPlayerPacket> for Entity {
    fn from(p: &ClientboundAddPlayerPacket) -> Self {
        Self::new(
            p.id,
            p.uuid,
            Vec3 {
                x: p.x,
                y: p.y,
                z: p.z,
            },
        )
    }
}
