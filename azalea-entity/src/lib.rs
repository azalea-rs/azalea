#![doc = include_str!("../README.md")]
#![allow(clippy::derived_hash_with_manual_eq)]

pub mod attributes;
mod data;
pub mod dimensions;
mod effects;
pub mod inventory;
#[cfg(feature = "bevy_ecs")]
pub mod metadata;
pub mod mining;
pub mod particle;
#[cfg(feature = "bevy_ecs")]
mod plugin;
pub mod vec_delta_codec;

use std::{
    f64::consts::PI,
    fmt::{self, Debug},
    hash::{Hash, Hasher},
};

pub use attributes::Attributes;
use azalea_block::fluid_state::FluidKind;
use azalea_buf::AzBuf;
use azalea_core::{
    aabb::Aabb,
    math,
    position::{BlockPos, ChunkPos, Vec3},
};
use azalea_registry::builtin::EntityKind;
pub use data::*;
use derive_more::{Deref, DerefMut};
pub use effects::{ActiveEffects, MobEffectData};
use uuid::Uuid;
use vec_delta_codec::VecDeltaCodec;

use self::attributes::AttributeInstance;
use crate::dimensions::EntityDimensions;
#[cfg(feature = "bevy_ecs")]
pub use crate::plugin::*;

pub fn move_relative(
    physics: &mut Physics,
    direction: LookDirection,
    speed: f32,
    acceleration: Vec3,
) {
    let input_vector = input_vector(direction, speed, acceleration);
    physics.velocity += input_vector;
}

pub fn input_vector(direction: LookDirection, speed: f32, acceleration: Vec3) -> Vec3 {
    let distance = acceleration.length_squared();
    if distance < 1.0e-7 {
        return Vec3::ZERO;
    }
    let acceleration = if distance > 1. {
        acceleration.normalize()
    } else {
        acceleration
    }
    .scale(speed as f64);
    let y_rot = math::sin(direction.y_rot * (PI / 180.) as f32);
    let x_rot = math::cos(direction.y_rot * (PI / 180.) as f32);
    Vec3 {
        x: acceleration.x * (x_rot as f64) - acceleration.z * (y_rot as f64),
        y: acceleration.y,
        z: acceleration.z * (x_rot as f64) + acceleration.x * (y_rot as f64),
    }
}

pub fn view_vector(look_direction: LookDirection) -> Vec3 {
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

/// The Minecraft UUID of the entity.
///
/// For players, this is their actual player UUID, and for other entities it's
/// just random.
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
#[derive(Clone, Copy, Default, Deref, DerefMut, PartialEq)]
pub struct EntityUuid(Uuid);
impl EntityUuid {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}
impl Debug for EntityUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0).fmt(f)
    }
}

/// The position of the entity right now.
///
/// If this is being used as an ECS component then you are free to modify it,
/// because there are systems that will update the indexes automatically.
///
/// Its value is set to a default of [`Vec3::ZERO`] when it receives the login
/// packet, its true position may be set ticks later.
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
#[derive(Clone, Copy, Debug, Default, Deref, DerefMut, PartialEq)]
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

/// The direction that an entity is looking, in degrees.
///
/// To avoid flagging anticheats, consider using [`Self::update`] when updating
/// the values of this struct.
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq)]
pub struct LookDirection {
    /// Left and right. AKA yaw. In degrees.
    y_rot: f32,
    /// Up and down. AKA pitch. In degrees.
    x_rot: f32,
}

impl LookDirection {
    /// Create a new look direction and clamp the `x_rot` to the allowed values.
    pub fn new(y_rot: f32, x_rot: f32) -> Self {
        Self { y_rot, x_rot }.clamped()
    }
    /// Returns yaw (left and right) in degrees.
    ///
    /// Minecraft allows this to go outside of ±360°, so it won't necessarily be
    /// in any range.
    pub fn y_rot(&self) -> f32 {
        self.y_rot
    }
    /// Returns pitch (up and down) in degrees.
    ///
    /// Clamped to ±90°.
    pub fn x_rot(&self) -> f32 {
        self.x_rot
    }

    /// Update this look direction to the new value.
    ///
    /// This handles relative rotations correctly and with the default Minecraft
    /// sensitivity to avoid triggering anticheats.
    pub fn update(&mut self, new: LookDirection) {
        self.update_with_sensitivity(new, 1.);
    }
    /// Update the `y_rot` (yaw) to the given value, in degrees.
    ///
    /// This is a shortcut for [`Self::update`] while keeping the `x_rot` the
    /// same.
    pub fn update_y_rot(&mut self, new_y_rot: f32) {
        self.update(LookDirection {
            y_rot: new_y_rot,
            x_rot: self.x_rot,
        });
    }
    /// Update the `x_rot` (pitch) to the given value, in degrees.
    ///
    /// This is a shortcut for [`Self::update`] while keeping the `y_rot` the
    /// same.
    pub fn update_x_rot(&mut self, new_x_rot: f32) {
        self.update(LookDirection {
            y_rot: self.y_rot,
            x_rot: new_x_rot,
        });
    }

    /// Update this look direction to the new value, using the given
    /// sensitivity value.
    ///
    /// Consider using [`Self::update`] instead, which uses 1.0 as the
    /// sensitivity (equivalent to 100% sensitivity in Minecraft).
    pub fn update_with_sensitivity(&mut self, new: LookDirection, sensitivity: f32) {
        let mut delta_y_rot = new.y_rot.rem_euclid(360.) - self.y_rot.rem_euclid(360.);
        let delta_x_rot = new.x_rot - self.x_rot;

        if delta_y_rot > 180. {
            delta_y_rot -= 360.;
        } else if delta_y_rot < -180. {
            delta_y_rot += 360.;
        }

        let sensitivity = sensitivity * 0.15;
        let delta_y_rot = (delta_y_rot / sensitivity).round() * sensitivity;
        let delta_x_rot = (delta_x_rot / sensitivity).round() * sensitivity;

        self.y_rot += delta_y_rot;
        self.x_rot += delta_x_rot;
    }

    /// Force the [`Self::x_rot`] to be between -90 and 90 degrees, and return
    /// the new look direction.
    #[must_use]
    pub fn clamped(mut self) -> Self {
        self.x_rot = self.x_rot.clamp(-90., 90.);
        self
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
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
#[derive(Clone, Debug, Default)]
pub struct Physics {
    /// How fast the entity is moving. Sometimes referred to as the delta
    /// movement.
    ///
    /// Note that our Y velocity will be approximately -0.0784 when we're on the
    /// ground due to how Minecraft applies gravity.
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

    /// The bounding box of the entity.
    ///
    /// This is more than just width and height, unlike dimensions.
    pub bounding_box: Aabb,

    pub has_impulse: bool,

    pub horizontal_collision: bool,
    // TODO: implement minor_horizontal_collision
    pub minor_horizontal_collision: bool,
    pub vertical_collision: bool,

    pub water_fluid_height: f64,
    pub lava_fluid_height: f64,
    pub was_touching_water: bool,

    pub fall_distance: f64,
    // TODO: implement remaining_fire_ticks
    pub remaining_fire_ticks: i32,
}

impl Physics {
    pub fn new(dimensions: &EntityDimensions, pos: Vec3) -> Self {
        Self {
            velocity: Vec3::ZERO,
            vec_delta_codec: VecDeltaCodec::new(pos),

            old_position: pos,

            x_acceleration: 0.,
            y_acceleration: 0.,
            z_acceleration: 0.,

            on_ground: false,
            last_on_ground: false,

            no_jump_delay: 0,

            bounding_box: dimensions.make_bounding_box(pos),

            has_impulse: false,

            horizontal_collision: false,
            minor_horizontal_collision: false,
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

    pub fn set_old_pos(&mut self, pos: Position) {
        self.old_position = *pos;
    }
}

impl Attributes {
    pub fn new(_entity_kind: EntityKind) -> Self {
        // TODO: do the correct defaults for everything, some
        // entities have different defaults
        Attributes {
            movement_speed: AttributeInstance::new(0.1f32 as f64),
            sneaking_speed: AttributeInstance::new(0.3),
            attack_speed: AttributeInstance::new(4.0),
            water_movement_efficiency: AttributeInstance::new(0.0),
            mining_efficiency: AttributeInstance::new(0.0),
            block_interaction_range: AttributeInstance::new(4.5),
            entity_interaction_range: AttributeInstance::new(3.0),
            step_height: AttributeInstance::new(0.6),
            block_break_speed: AttributeInstance::new(1.0),
        }
    }
}

/// The abilities that a player has, such as flying or being able to instantly
/// break blocks.
///
/// This should only be present on local players.
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
#[derive(Clone, Debug, Default)]
pub struct PlayerAbilities {
    pub invulnerable: bool,
    pub flying: bool,
    pub can_fly: bool,
    /// Whether the player can instantly break blocks and can duplicate blocks
    /// in their inventory.
    pub instant_break: bool,

    pub flying_speed: f32,
    /// Used for the fov
    pub walking_speed: f32,
}

/// The type of fluid that is at an entity's eye position, while also accounting
/// for fluid height.
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
#[derive(Clone, Copy, Debug, Deref, DerefMut, PartialEq)]
pub struct FluidOnEyes(FluidKind);
