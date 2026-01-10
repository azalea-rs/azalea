use azalea_block::fluid_state::FluidKind;
use azalea_core::position::{ChunkPos, Vec3};
use azalea_registry::{builtin::EntityKind, identifier::Identifier};
use azalea_world::InstanceName;
use bevy_ecs::{bundle::Bundle, component::Component};
use derive_more::{Deref, DerefMut};
use uuid::Uuid;

use crate::{
    ActiveEffects, Attributes, EntityUuid, FluidOnEyes, Jumping, LastSentPosition, LookDirection,
    Physics, Position, dimensions::EntityDimensions, indexing::EntityChunkPos,
};

/// A bundle of components that every entity has.
///
/// This doesn't contain metadata; that has to be added separately.
#[derive(Bundle)]
pub struct EntityBundle {
    pub kind: EntityKindComponent,
    pub uuid: EntityUuid,
    pub world_name: InstanceName,
    pub position: Position,
    pub last_sent_position: LastSentPosition,

    pub chunk_pos: EntityChunkPos,

    pub physics: Physics,
    pub direction: LookDirection,
    pub dimensions: EntityDimensions,
    pub attributes: Attributes,
    pub jumping: Jumping,
    pub crouching: Crouching,
    pub fluid_on_eyes: FluidOnEyes,
    pub on_climbable: OnClimbable,
    pub active_effects: ActiveEffects,
}

impl EntityBundle {
    pub fn new(uuid: Uuid, pos: Vec3, kind: EntityKind, world_name: Identifier) -> Self {
        let dimensions = EntityDimensions::from(kind);

        Self {
            kind: EntityKindComponent(kind),
            uuid: EntityUuid(uuid),
            world_name: InstanceName(world_name),
            position: Position(pos),
            chunk_pos: EntityChunkPos(ChunkPos::from(&pos)),
            last_sent_position: LastSentPosition(pos),
            physics: Physics::new(&dimensions, pos),
            dimensions,
            direction: LookDirection::default(),

            attributes: Attributes::new(EntityKind::Player),

            jumping: Jumping(false),
            crouching: Crouching(false),
            fluid_on_eyes: FluidOnEyes(FluidKind::Empty),
            on_climbable: OnClimbable(false),
            active_effects: ActiveEffects::default(),
        }
    }
}

/// Marker component for entities that are dead.
///
/// "Dead" means that the entity has 0 health.
#[derive(Clone, Component, Copy, Default)]
pub struct Dead;

/// A component NewType for [`EntityKind`].
///
/// Most of the time, you should be using `azalea_registry::EntityKind`
/// directly instead.
#[derive(Clone, Component, Copy, Debug, Deref, PartialEq)]
pub struct EntityKindComponent(pub EntityKind);

/// A marker component that signifies that this entity is "local" and shouldn't
/// be updated by other clients.
///
/// If this is for a client then all of our clients will have this.
///
/// This component is not removed from clients when they disconnect.
#[derive(Clone, Component, Copy, Debug, Default)]
pub struct LocalEntity;

impl FluidOnEyes {
    pub fn new(fluid: FluidKind) -> Self {
        Self(fluid)
    }
}

#[derive(Clone, Component, Copy, Debug, Deref, DerefMut, PartialEq)]
pub struct OnClimbable(bool);

/// A component that indicates whether the player is currently sneaking.
///
/// If the entity isn't a local player, then this is just a shortcut for
/// checking if the [`Pose`] is `Crouching`.
///
/// If you need to modify this value, use
/// `azalea_client::PhysicsState::trying_to_crouch` or `Client::set_crouching`
/// instead.
///
/// [`Pose`]: crate::data::Pose
#[derive(Clone, Component, Copy, Default, Deref, DerefMut)]
pub struct Crouching(bool);

/// A component that indicates whether the client has loaded.
///
/// This is updated by a system in `azalea-client`.
#[derive(Component)]
pub struct HasClientLoaded;
