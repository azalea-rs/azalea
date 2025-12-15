use azalea_buf::AzBuf;
use azalea_core::{delta::LpVec3, position::Vec3};
use azalea_entity::{EntityBundle, metadata::apply_default_metadata};
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::{builtin::EntityKind, identifier::Identifier};
use azalea_world::MinecraftEntityId;
use uuid::Uuid;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundAddEntity {
    /// The numeric ID of the entity being added to the world.
    #[var]
    pub id: MinecraftEntityId,
    pub uuid: Uuid,
    pub entity_type: EntityKind,
    pub position: Vec3,
    pub movement: LpVec3,
    pub x_rot: i8,
    pub y_rot: i8,
    pub y_head_rot: i8,
    /// The entity's "object data". This is unused for most entities.
    ///
    /// Projectiles and fishing hooks treat this like a [`MinecraftEntityId`].
    /// Falling blocks treat it as a [`BlockState`](azalea_block::BlockState).
    /// Other entities may treat it as another enum variant.
    ///
    /// See [the wiki](https://minecraft.wiki/w/Java_Edition_protocol/Object_data)
    /// for more information about this field.
    #[var]
    pub data: i32,
}

impl ClientboundAddEntity {
    /// Make the entity into a bundle that can be inserted into the ECS.
    ///
    /// You must apply the metadata after inserting the bundle with
    /// [`Self::apply_metadata`].
    pub fn as_entity_bundle(&self, world_name: Identifier) -> EntityBundle {
        EntityBundle::new(self.uuid, self.position, self.entity_type, world_name)
    }

    /// Apply the default metadata for the given entity.
    pub fn apply_metadata(&self, entity: &mut bevy_ecs::system::EntityCommands) {
        apply_default_metadata(entity, self.entity_type);
    }
}
