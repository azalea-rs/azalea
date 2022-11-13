use azalea_buf::McBuf;
use azalea_core::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::entity::{EntityData, EntityMetadata};
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundAddEntityPacket {
    /// The id of the entity.
    #[var]
    pub id: u32,
    pub uuid: Uuid,
    pub entity_type: azalea_registry::EntityType,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_rot: i8,
    pub y_rot: i8,
    pub y_head_rot: i8,
    #[var]
    pub data: i32,
    pub x_vel: i16,
    pub y_vel: i16,
    pub z_vel: i16,
}

impl From<&ClientboundAddEntityPacket> for EntityData {
    fn from(p: &ClientboundAddEntityPacket) -> Self {
        Self::new(
            p.uuid,
            Vec3 {
                x: p.x,
                y: p.y,
                z: p.z,
            },
            // default metadata for the entity type
            EntityMetadata::from(p.entity_type),
        )
    }
}
