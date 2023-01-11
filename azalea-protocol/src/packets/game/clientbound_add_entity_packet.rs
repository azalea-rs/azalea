use azalea_buf::McBuf;
use azalea_core::{ResourceLocation, Vec3};
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::entity::{metadata::apply_default_metadata, EntityBundle};
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundAddEntityPacket {
    /// The id of the entity.
    #[var]
    pub id: u32,
    pub uuid: Uuid,
    pub entity_type: azalea_registry::EntityKind,
    pub position: Vec3,
    pub x_rot: i8,
    pub y_rot: i8,
    pub y_head_rot: i8,
    #[var]
    pub data: i32,
    pub x_vel: i16,
    pub y_vel: i16,
    pub z_vel: i16,
}

// impl From<&ClientboundAddEntityPacket> for EntityData {
//     fn from(p: &ClientboundAddEntityPacket) -> Self {
//         Self::new(
//             p.uuid,
//             Vec3 {
//                 x: p.x,
//                 y: p.y,
//                 z: p.z,
//             },
//             // default metadata for the entity type
//             EntityMetadata::from(p.entity_type),
//         )
//     }
// }

impl ClientboundAddEntityPacket {
    /// Make the entity into a bundle that can be inserted into the ECS. You
    /// must apply the metadata after inserting the bundle with
    /// [`Self::apply_metadata`].
    pub fn as_entity_bundle(&self, world_name: ResourceLocation) -> EntityBundle {
        EntityBundle::new(self.uuid, self.position, self.entity_type, world_name)
    }

    /// Apply the default metadata for the given entity.
    pub fn apply_metadata(&self, entity: &mut bevy_ecs::system::EntityCommands) {
        apply_default_metadata(entity, self.entity_type);
    }
}
