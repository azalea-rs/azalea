pub mod attributes;
mod data;
mod dimensions;
pub mod metadata;

use self::{attributes::AttributeInstance, metadata::UpdateMetadataError};
pub use attributes::Attributes;
use azalea_block::BlockState;
use azalea_core::{BlockPos, ChunkPos, Vec3, AABB};
use bevy_ecs::{
    bundle::Bundle, component::Component, query::Changed, system::Query, world::EntityMut,
};
pub use data::*;
use derive_more::{Deref, DerefMut};
pub use dimensions::EntityDimensions;
use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;

/// A lightweight identifier of an entity.
///
/// Don't rely on the index of this being the same as a Minecraft entity id!
/// (unless you're implementin a server, in which case you can decide your
/// entity ids however you want)
///
/// If you want to refer to a Minecraft entity id, use [`MinecraftEntityId`].
pub type Entity = bevy_ecs::entity::Entity;

/// An entity ID used by Minecraft. These are not guaranteed to be unique in
/// shared worlds, that's what [`Entity`] is for.
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub struct MinecraftEntityId(pub u32);
impl std::hash::Hash for MinecraftEntityId {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write_u32(self.0)
    }
}
impl nohash_hasher::IsEnabled for MinecraftEntityId {}

/// Sets the position of the entity. This doesn't update the cache in
/// azalea-world, and should only be used within azalea-world!
///
/// # Safety
/// Cached position in the world must be updated.
pub fn update_bounding_box(query: Query<(&mut Position, &Physics), Changed<Position>>) {
    for (mut position, physics) in query.iter_mut() {
        let bounding_box = physics.dimensions.make_bounding_box(&position);
        physics.bounding_box = bounding_box;
    }
}

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

/// Apply the given metadata items to the entity. Everything that isn't
/// included in items will be left unchanged.
pub fn apply_metadata(
    ecs: bevy_ecs::world::World,
    entity: &mut bevy_ecs::world::EntityMut,
    items: Vec<EntityDataItem>,
) -> Result<(), UpdateMetadataError> {
    metadata::apply_metadata(entity, items)
}

pub fn make_bounding_box(pos: &Position, physics: &Physics) -> AABB {
    physics.dimensions.make_bounding_box(&pos)
}

/// Get the position of the block below the entity, but a little lower.
pub fn on_pos_legacy(world: &World, position: &Position, physics: &Physics) -> BlockPos {
    on_pos(world, position, physics, 0.2)
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
pub fn on_pos(world: &World, pos: &Position, physics: &Physics, offset: f32) -> BlockPos {
    let x = pos.x.floor() as i32;
    let y = (pos.y - offset as f64).floor() as i32;
    let z = pos.z.floor() as i32;
    let pos = BlockPos { x, y, z };

    // TODO: check if block below is a fence, wall, or fence gate
    let block_pos = pos.down(1);
    let block_state = world.get_block_state(&block_pos);
    if block_state == Some(BlockState::Air) {
        let block_pos_below = block_pos.down(1);
        let block_state_below = world.get_block_state(&block_pos_below);
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

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct EntityUuid(Uuid);

/// The position of the entity right now.
/// This can be changed with unsafe_move, but the correct way is with
/// world.move_entity
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

/// The position of the entity last tick.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Deref, DerefMut)]
pub struct LastPosition(Vec3);
impl From<LastPosition> for ChunkPos {
    fn from(value: LastPosition) -> Self {
        ChunkPos::from(&value.0)
    }
}
impl From<LastPosition> for BlockPos {
    fn from(value: LastPosition) -> Self {
        BlockPos::from(&value.0)
    }
}

/// Set the [`LastPosition`] component to the current [`Position`] component.
/// This should happen at the end of every tick.
pub fn update_last_position(
    mut query: Query<(&mut Position, &mut LastPosition), Changed<Position>>,
) {
    for (mut position, mut last_position) in query.iter_mut() {
        *last_position = LastPosition(**position);
    }
}

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

    /// Whether the entity will try to jump every tick
    /// (equivalent to the space key being held down in vanilla).
    pub jumping: bool,

    pub has_impulse: bool,
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
    pub position: Position,
    pub last_position: LastPosition,
    pub physics: Physics,
    pub attributes: Attributes,
}

impl EntityBundle {
    pub fn new(uuid: Uuid, pos: Vec3, kind: azalea_registry::EntityKind) -> Self {
        let dimensions = EntityDimensions {
            width: 0.6,
            height: 1.8,
        };

        Self {
            kind: EntityKind(kind),
            uuid: EntityUuid(uuid),
            position: Position(pos),
            last_position: Position(pos),
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

                jumping: false,
            },

            attributes: Attributes {
                // TODO: do the correct defaults for everything, some
                // entities have different defaults
                speed: AttributeInstance::new(0.1),
            },
        }
    }
}

/// A bundle of the components that are always present for a player.
#[derive(Bundle)]
pub struct PlayerBundle {
    pub entity: EntityBundle,
    pub metadata: metadata::PlayerMetadataBundle,
}

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
