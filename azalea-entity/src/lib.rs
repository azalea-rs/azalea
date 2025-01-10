#![allow(clippy::derived_hash_with_manual_eq)]

pub mod attributes;
mod data;
mod dimensions;
mod effects;
mod enchantments;
pub mod metadata;
pub mod mining;
pub mod particle;
mod plugin;
pub mod vec_delta_codec;

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

pub use attributes::Attributes;
use azalea_block::{fluid_state::FluidKind, BlockState};
use azalea_buf::AzBuf;
use azalea_core::{
    aabb::AABB,
    math,
    position::{BlockPos, ChunkPos, Vec3},
    resource_location::ResourceLocation,
};
use azalea_world::{ChunkStorage, InstanceName};
use bevy_ecs::{bundle::Bundle, component::Component};
pub use data::*;
use derive_more::{Deref, DerefMut};
pub use dimensions::EntityDimensions;
use plugin::indexing::EntityChunkPos;
use uuid::Uuid;
use vec_delta_codec::VecDeltaCodec;

use self::attributes::AttributeInstance;
pub use crate::plugin::*;

pub fn move_relative(
    physics: &mut Physics,
    direction: &LookDirection,
    speed: f32,
    acceleration: &Vec3,
) {
    let input_vector = input_vector(direction, speed, acceleration);
    physics.velocity += input_vector;
}

pub fn input_vector(direction: &LookDirection, speed: f32, acceleration: &Vec3) -> Vec3 {
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
    let y_rot = math::sin(direction.y_rot * 0.017453292f32);
    let x_rot = math::cos(direction.y_rot * 0.017453292f32);
    Vec3 {
        x: acceleration.x * (x_rot as f64) - acceleration.z * (y_rot as f64),
        y: acceleration.y,
        z: acceleration.z * (x_rot as f64) + acceleration.x * (y_rot as f64),
    }
}

pub fn view_vector(look_direction: &LookDirection) -> Vec3 {
    let x_rot = look_direction.x_rot * 0.017453292;
    let y_rot = -look_direction.y_rot * 0.017453292;
    let y_rot_cos = math::cos(y_rot);
    let y_rot_sin = math::sin(y_rot);
    let x_rot_cos = math::cos(x_rot);
    let x_rot_sin = math::sin(x_rot);
    Vec3 {
        x: (y_rot_sin * x_rot_cos) as f64,
        y: (-x_rot_sin) as f64,
        z: (y_rot_cos * x_rot_cos) as f64,
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
#[derive(Component, Deref, DerefMut, Clone, Copy, Default)]
pub struct EntityUuid(Uuid);
impl EntityUuid {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}
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
impl Position {
    pub fn new(pos: Vec3) -> Self {
        Self(pos)
    }
}
impl From<&Position> for Vec3 {
    fn from(value: &Position) -> Self {
        value.0
    }
}
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

/// The second most recent position of the entity that was sent over the
/// network. This is currently only updated for our own local player entities.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Deref, DerefMut)]
pub struct LastSentPosition(Vec3);
impl From<&LastSentPosition> for Vec3 {
    fn from(value: &LastSentPosition) -> Self {
        value.0
    }
}
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

/// A component for entities that can jump.
///
/// If this is true, the entity will try to jump every tick. It's equivalent to
/// the space key being held in vanilla.
#[derive(Debug, Component, Copy, Clone, Deref, DerefMut, Default)]
pub struct Jumping(bool);

/// A component that contains the direction an entity is looking.
#[derive(Debug, Component, Copy, Clone, Default, PartialEq, AzBuf)]
pub struct LookDirection {
    /// Left and right. Aka yaw.
    pub y_rot: f32,
    /// Up and down. Aka pitch.
    pub x_rot: f32,
}

impl LookDirection {
    pub fn new(y_rot: f32, x_rot: f32) -> Self {
        Self { y_rot, x_rot }
    }
}

impl From<LookDirection> for (f32, f32) {
    fn from(value: LookDirection) -> Self {
        (value.y_rot, value.x_rot)
    }
}
impl From<(f32, f32)> for LookDirection {
    fn from((y_rot, x_rot): (f32, f32)) -> Self {
        Self { y_rot, x_rot }
    }
}

impl Hash for LookDirection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.y_rot.to_bits().hash(state);
        self.x_rot.to_bits().hash(state);
    }
}
impl Eq for LookDirection {}

/// The physics data relating to the entity, such as position, velocity, and
/// bounding box.
#[derive(Debug, Component, Clone, Default)]
pub struct Physics {
    /// How fast the entity is moving.
    ///
    /// Sometimes referred to as the delta movement.
    pub velocity: Vec3,
    pub vec_delta_codec: VecDeltaCodec,

    /// The position of the entity before it moved this tick.
    ///
    /// This is set immediately before physics is done.
    pub old_position: Vec3,

    /// The acceleration here is the force that will be attempted to be added to
    /// the entity's velocity next tick.
    ///
    /// You should typically not set this yourself, since it's controlled by how
    /// the entity is trying to move.
    pub x_acceleration: f32,
    pub y_acceleration: f32,
    pub z_acceleration: f32,

    on_ground: bool,
    last_on_ground: bool,

    /// The number of ticks until we jump again, if the jump key is being held.
    ///
    /// This must be 0 for us to be able to jump. Sets to 10 when we do a jump
    /// and sets to 0 if we're not trying to jump.
    pub no_jump_delay: u32,

    /// The width and height of the entity.
    pub dimensions: EntityDimensions,
    /// The bounding box of the entity. This is more than just width and height,
    /// unlike dimensions.
    pub bounding_box: AABB,

    pub has_impulse: bool,

    pub horizontal_collision: bool,
    // pub minor_horizontal_collision: bool,
    pub vertical_collision: bool,

    pub water_fluid_height: f64,
    pub lava_fluid_height: f64,
    pub was_touching_water: bool,

    // TODO: implement fall_distance
    pub fall_distance: f32,
    // TODO: implement remaining_fire_ticks
    pub remaining_fire_ticks: i32,
}

impl Physics {
    pub fn new(dimensions: EntityDimensions, pos: Vec3) -> Self {
        Self {
            velocity: Vec3::default(),
            vec_delta_codec: VecDeltaCodec::new(pos),

            old_position: pos,

            x_acceleration: 0.,
            y_acceleration: 0.,
            z_acceleration: 0.,

            on_ground: false,
            last_on_ground: false,

            no_jump_delay: 0,

            bounding_box: dimensions.make_bounding_box(&pos),
            dimensions,

            has_impulse: false,

            horizontal_collision: false,
            vertical_collision: false,

            water_fluid_height: 0.,
            lava_fluid_height: 0.,
            was_touching_water: false,

            fall_distance: 0.,
            remaining_fire_ticks: 0,
        }
    }

    pub fn on_ground(&self) -> bool {
        self.on_ground
    }
    /// Updates [`Self::on_ground`] and [`Self::last_on_ground`].
    pub fn set_on_ground(&mut self, on_ground: bool) {
        self.last_on_ground = self.on_ground;
        self.on_ground = on_ground;
    }

    /// The last value of the on_ground value.
    ///
    /// This is used by Azalea internally for physics, it might not work as you
    /// expect since it can be influenced by packets sent by the server.
    pub fn last_on_ground(&self) -> bool {
        self.last_on_ground
    }
    pub fn set_last_on_ground(&mut self, last_on_ground: bool) {
        self.last_on_ground = last_on_ground;
    }

    pub fn reset_fall_distance(&mut self) {
        self.fall_distance = 0.;
    }
    pub fn clear_fire(&mut self) {
        self.remaining_fire_ticks = 0;
    }

    pub fn is_in_water(&self) -> bool {
        self.was_touching_water
    }
    pub fn is_in_lava(&self) -> bool {
        // TODO: also check `!this.firstTick &&`
        self.lava_fluid_height > 0.
    }

    pub fn set_old_pos(&mut self, pos: &Position) {
        self.old_position = **pos;
    }
}

/// Marker component for entities that are dead.
///
/// "Dead" means that the entity has 0 health.
#[derive(Component, Copy, Clone, Default)]
pub struct Dead;

/// A component that contains the offset of the entity's eyes from the entity
/// coordinates.
///
/// This is used to calculate the camera position for players, when spectating
/// an entity, and when raycasting from the entity.
#[derive(Component, Clone, Copy, Debug, PartialEq, Deref, DerefMut)]
pub struct EyeHeight(f32);
impl EyeHeight {
    pub fn new(height: f32) -> Self {
        Self(height)
    }
}
impl From<EyeHeight> for f32 {
    fn from(value: EyeHeight) -> Self {
        value.0
    }
}
impl From<EyeHeight> for f64 {
    fn from(value: EyeHeight) -> Self {
        value.0 as f64
    }
}
impl From<&EyeHeight> for f32 {
    fn from(value: &EyeHeight) -> Self {
        value.0
    }
}
impl From<&EyeHeight> for f64 {
    fn from(value: &EyeHeight) -> Self {
        value.0 as f64
    }
}

/// A component NewType for [`azalea_registry::EntityKind`].
///
/// Most of the time, you should be using `azalea_registry::EntityKind`
/// directly instead.
#[derive(Component, Clone, Copy, Debug, PartialEq, Deref)]
pub struct EntityKind(pub azalea_registry::EntityKind);

/// A bundle of components that every entity has. This doesn't contain metadata,
/// that has to be added separately.
#[derive(Bundle)]
pub struct EntityBundle {
    pub kind: EntityKind,
    pub uuid: EntityUuid,
    pub world_name: InstanceName,
    pub position: Position,
    pub last_sent_position: LastSentPosition,

    pub chunk_pos: EntityChunkPos,

    pub physics: Physics,
    pub direction: LookDirection,
    pub eye_height: EyeHeight,
    pub attributes: Attributes,
    pub jumping: Jumping,
    pub fluid_on_eyes: FluidOnEyes,
    pub on_climbable: OnClimbable,
}

impl EntityBundle {
    pub fn new(
        uuid: Uuid,
        pos: Vec3,
        kind: azalea_registry::EntityKind,
        world_name: ResourceLocation,
    ) -> Self {
        // TODO: get correct entity dimensions by having them codegen'd somewhere
        let dimensions = EntityDimensions {
            width: 0.6,
            height: 1.8,
        };
        let eye_height = dimensions.height * 0.85;

        Self {
            kind: EntityKind(kind),
            uuid: EntityUuid(uuid),
            world_name: InstanceName(world_name),
            position: Position(pos),
            chunk_pos: EntityChunkPos(ChunkPos::from(&pos)),
            last_sent_position: LastSentPosition(pos),
            physics: Physics::new(dimensions, pos),
            eye_height: EyeHeight(eye_height),
            direction: LookDirection::default(),

            attributes: Attributes {
                // TODO: do the correct defaults for everything, some
                // entities have different defaults
                speed: AttributeInstance::new(0.1),
                attack_speed: AttributeInstance::new(4.0),
                water_movement_efficiency: AttributeInstance::new(0.0),
            },

            jumping: Jumping(false),
            fluid_on_eyes: FluidOnEyes(FluidKind::Empty),
            on_climbable: OnClimbable(false),
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
///
/// If this is for a client then all of our clients will have this.
#[derive(Component, Clone)]
pub struct LocalEntity;

#[derive(Component, Clone, Debug, PartialEq, Deref, DerefMut)]
pub struct FluidOnEyes(FluidKind);

impl FluidOnEyes {
    pub fn new(fluid: FluidKind) -> Self {
        Self(fluid)
    }
}

#[derive(Component, Clone, Debug, PartialEq, Deref, DerefMut)]
pub struct OnClimbable(bool);
