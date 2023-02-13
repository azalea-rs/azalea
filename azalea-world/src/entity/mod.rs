#![allow(clippy::derived_hash_with_manual_eq)]

pub mod attributes;
mod data;
mod dimensions;
mod info;
pub mod metadata;

use crate::ChunkStorage;

use self::{attributes::AttributeInstance, metadata::Health};
pub use attributes::Attributes;
use azalea_block::BlockState;
use azalea_core::{BlockPos, ChunkPos, ResourceLocation, Vec3, AABB};
use azalea_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::Changed,
    system::{Commands, Query},
};
pub use data::*;
use derive_more::{Deref, DerefMut};
pub use dimensions::{update_bounding_box, EntityDimensions};
pub use info::{EntityInfos, EntityPlugin, LoadedBy, PartialEntityInfos, RelativeEntityUpdate};
use std::fmt::Debug;
use uuid::Uuid;

/// An entity ID used by Minecraft. These are not guaranteed to be unique in
/// shared worlds, that's what [`Entity`] is for.
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub struct MinecraftEntityId(pub u32);
impl std::hash::Hash for MinecraftEntityId {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write_u32(self.0);
    }
}
impl nohash_hasher::IsEnabled for MinecraftEntityId {}
pub fn set_rotation(physics: &mut Physics, y_rot: f32, x_rot: f32) {
    physics.y_rot = y_rot % 360.0;
    physics.x_rot = x_rot.clamp(-90.0, 90.0) % 360.0;
    // TODO: minecraft also sets yRotO and xRotO to xRot and yRot ... but
    // idk what they're used for so
}

pub fn move_relative(physics: &mut Physics, speed: f32, acceleration: &Vec3) {
    let input_vector = input_vector(physics, speed, acceleration);
    physics.delta += input_vector;
}

pub fn input_vector(physics: &mut Physics, speed: f32, acceleration: &Vec3) -> Vec3 {
    let distance = acceleration.length_squared();
    if distance < 1.0E-7 {
        return Vec3::default();
    }
    let acceleration = if distance > 1.0 {
        acceleration.normalize()
    } else {
        *acceleration
    }
    .scale(speed as f64);
    let y_rot = f32::sin(physics.y_rot * 0.017453292f32);
    let x_rot = f32::cos(physics.y_rot * 0.017453292f32);
    Vec3 {
        x: acceleration.x * (x_rot as f64) - acceleration.z * (y_rot as f64),
        y: acceleration.y,
        z: acceleration.z * (x_rot as f64) + acceleration.x * (y_rot as f64),
    }
}

/// Get the position of the block below the entity, but a little lower.
pub fn on_pos_legacy(chunk_storage: &ChunkStorage, position: &Position) -> BlockPos {
    on_pos(0.2, chunk_storage, position)
}

// int x = Mth.floor(this.position.x);
// int y = Mth.floor(this.position.y - (double)var1);
// int z = Mth.floor(this.position.z);
// BlockPos var5 = new BlockPos(x, y, z);
// if (this.level.getBlockState(var5).isAir()) {
//    BlockPos var6 = var5.below();
//    BlockState var7 = this.level.getBlockState(var6);
//    if (var7.is(BlockTags.FENCES) || var7.is(BlockTags.WALLS) ||
// var7.getBlock() instanceof FenceGateBlock) {       return var6;
//    }
// }
// return var5;
pub fn on_pos(offset: f32, chunk_storage: &ChunkStorage, pos: &Position) -> BlockPos {
    let x = pos.x.floor() as i32;
    let y = (pos.y - offset as f64).floor() as i32;
    let z = pos.z.floor() as i32;
    let pos = BlockPos { x, y, z };

    // TODO: check if block below is a fence, wall, or fence gate
    let block_pos = pos.down(1);
    let block_state = chunk_storage.get_block_state(&block_pos);
    if block_state == Some(BlockState::AIR) {
        let block_pos_below = block_pos.down(1);
        let block_state_below = chunk_storage.get_block_state(&block_pos_below);
        if let Some(_block_state_below) = block_state_below {
            // if block_state_below.is_fence()
            //     || block_state_below.is_wall()
            //     || block_state_below.is_fence_gate()
            // {
            //     return block_pos_below;
            // }
        }
    }

    pos
}

/// The Minecraft UUID of the entity. For players, this is their actual player
/// UUID, and for other entities it's just random.
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct EntityUuid(Uuid);
impl Debug for EntityUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0).fmt(f)
    }
}

/// The position of the entity right now.
///
/// You are free to change this; there's systems that update the indexes
/// automatically.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Deref, DerefMut)]
pub struct Position(Vec3);
impl From<Position> for ChunkPos {
    fn from(value: Position) -> Self {
        ChunkPos::from(&value.0)
    }
}
impl From<Position> for BlockPos {
    fn from(value: Position) -> Self {
        BlockPos::from(&value.0)
    }
}
impl From<&Position> for ChunkPos {
    fn from(value: &Position) -> Self {
        ChunkPos::from(value.0)
    }
}
impl From<&Position> for BlockPos {
    fn from(value: &Position) -> Self {
        BlockPos::from(value.0)
    }
}

/// The last position of the entity that was sent to the network.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Deref, DerefMut)]
pub struct LastSentPosition(Vec3);
impl From<LastSentPosition> for ChunkPos {
    fn from(value: LastSentPosition) -> Self {
        ChunkPos::from(&value.0)
    }
}
impl From<LastSentPosition> for BlockPos {
    fn from(value: LastSentPosition) -> Self {
        BlockPos::from(&value.0)
    }
}
impl From<&LastSentPosition> for ChunkPos {
    fn from(value: &LastSentPosition) -> Self {
        ChunkPos::from(value.0)
    }
}
impl From<&LastSentPosition> for BlockPos {
    fn from(value: &LastSentPosition) -> Self {
        BlockPos::from(value.0)
    }
}

/// The name of the world the entity is in. If two entities share the same world
/// name, we assume they're in the same world.
#[derive(Component, Clone, Debug, PartialEq, Deref, DerefMut)]
pub struct WorldName(pub ResourceLocation);

/// A component for entities that can jump.
///
/// If this is true, the entity will try to jump every tick. (It's equivalent to
/// the space key being held in vanilla.)
#[derive(Debug, Component, Deref, DerefMut)]
pub struct Jumping(bool);

/// The physics data relating to the entity, such as position, velocity, and
/// bounding box.
#[derive(Debug, Component)]
pub struct Physics {
    pub delta: Vec3,

    /// X acceleration.
    pub xxa: f32,
    /// Y acceleration.
    pub yya: f32,
    /// Z acceleration.
    pub zza: f32,

    pub x_rot: f32,
    pub y_rot: f32,

    pub x_rot_last: f32,
    pub y_rot_last: f32,

    pub on_ground: bool,
    pub last_on_ground: bool,

    /// The width and height of the entity.
    pub dimensions: EntityDimensions,
    /// The bounding box of the entity. This is more than just width and height,
    /// unlike dimensions.
    pub bounding_box: AABB,

    pub has_impulse: bool,
}

/// Marker component for entities that are dead.
///
/// "Dead" means that the entity has 0 health.
#[derive(Component, Copy, Clone, Default)]
pub struct Dead;

/// System that adds the [`Dead`] marker component if an entity's health is set
/// to 0 (or less than 0). This will be present if an entity is doing the death
/// animation.
///
/// Entities that are dead can not be revived.
/// TODO: fact check this in-game by setting an entity's health to 0 and then
/// not 0
pub fn add_dead(mut commands: Commands, query: Query<(Entity, &Health), Changed<Health>>) {
    for (entity, health) in query.iter() {
        if **health <= 0.0 {
            commands.entity(entity).insert(Dead);
        }
    }
}

/// A component NewType for [`azalea_registry::EntityKind`].
///
/// Most of the time, you should be using `azalea_registry::EntityKind`
/// instead.
#[derive(Component, Clone, Copy, Debug, PartialEq, Deref)]
pub struct EntityKind(azalea_registry::EntityKind);

/// A bundle of components that every entity has. This doesn't contain metadata,
/// that has to be added separately.
#[derive(Bundle)]
pub struct EntityBundle {
    pub kind: EntityKind,
    pub uuid: EntityUuid,
    pub world_name: WorldName,
    pub position: Position,
    pub last_sent_position: LastSentPosition,
    pub physics: Physics,
    pub attributes: Attributes,
    pub jumping: Jumping,
}

impl EntityBundle {
    pub fn new(
        uuid: Uuid,
        pos: Vec3,
        kind: azalea_registry::EntityKind,
        world_name: ResourceLocation,
    ) -> Self {
        // TODO: get correct entity dimensions by having them codegened somewhere
        let dimensions = EntityDimensions {
            width: 0.6,
            height: 1.8,
        };

        Self {
            kind: EntityKind(kind),
            uuid: EntityUuid(uuid),
            world_name: WorldName(world_name),
            position: Position(pos),
            last_sent_position: LastSentPosition(pos),
            physics: Physics {
                delta: Vec3::default(),

                xxa: 0.,
                yya: 0.,
                zza: 0.,

                x_rot: 0.,
                y_rot: 0.,

                y_rot_last: 0.,
                x_rot_last: 0.,

                on_ground: false,
                last_on_ground: false,

                // TODO: have this be based on the entity type
                bounding_box: dimensions.make_bounding_box(&pos),
                dimensions,

                has_impulse: false,
            },

            attributes: Attributes {
                // TODO: do the correct defaults for everything, some
                // entities have different defaults
                speed: AttributeInstance::new(0.1),
            },

            jumping: Jumping(false),
        }
    }
}

/// A bundle of the components that are always present for a player.
#[derive(Bundle)]
pub struct PlayerBundle {
    pub entity: EntityBundle,
    pub metadata: metadata::PlayerMetadataBundle,
}

/// A marker component that signifies that this entity is "local" and shouldn't
/// be updated by other clients.
#[derive(Component)]
pub struct Local;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::PartialWorld;

//     #[test]
//     fn from_mut_entity_to_ref_entity() {
//         let mut world = PartialWorld::default();
//         let uuid = Uuid::from_u128(100);
//         world.add_entity(
//             0,
//             EntityData::new(
//                 uuid,
//                 Vec3::default(),
//                 EntityMetadata::Player(metadata::Player::default()),
//             ),
//         );
//         let entity: Entity = world.entity_mut(0).unwrap();
//         assert_eq!(entity.uuid, uuid);
//     }
// }
