pub mod indexing;
mod relative_updates;

use std::collections::HashSet;

use azalea_core::position::{BlockPos, ChunkPos, Vec3};
use azalea_world::{InstanceContainer, InstanceName, MinecraftEntityId};
use bevy_app::{App, Plugin, PreUpdate, Update};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};
use tracing::debug;

use crate::{
    metadata::Health, Dead, EyeHeight, FluidOnEyes, LocalEntity, LookDirection, Physics, Position,
};

use indexing::EntityUuidIndex;
pub use relative_updates::RelativeEntityUpdate;

/// A Bevy [`SystemSet`] for various types of entity updates.
#[derive(SystemSet, Debug, Hash, Eq, PartialEq, Clone)]
pub enum EntityUpdateSet {
    /// Create search indexes for entities.
    Index,
    /// Remove despawned entities from search indexes.
    Deindex,
}

/// Plugin handling some basic entity functionality.
pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        // entities get added pre-update
        // added to indexes during update (done by this plugin)
        // modified during update
        // despawned post-update (done by this plugin)
        app.add_systems(
            PreUpdate,
            indexing::remove_despawned_entities_from_indexes.in_set(EntityUpdateSet::Deindex),
        )
        .add_systems(
            Update,
            (
                (indexing::update_entity_chunk_positions).in_set(EntityUpdateSet::Index),
                (
                    relative_updates::debug_detect_updates_received_on_local_entities,
                    debug_new_entity,
                    add_dead,
                    clamp_look_direction,
                    update_fluid_on_eyes,
                ),
            ),
        )
        .add_systems(Update, update_bounding_box)
        .add_systems(PreUpdate, update_in_loaded_chunk)
        .init_resource::<EntityUuidIndex>();
    }
}

fn debug_new_entity(query: Query<(Entity, Option<&LocalEntity>), Added<MinecraftEntityId>>) {
    for (entity, local) in query.iter() {
        if local.is_some() {
            debug!("new local entity: {:?}", entity);
        } else {
            debug!("new entity: {:?}", entity);
        }
    }
}

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

pub fn update_fluid_on_eyes(
    mut query: Query<(&mut FluidOnEyes, &Position, &EyeHeight, &InstanceName)>,
    instance_container: Res<InstanceContainer>,
) {
    for (mut fluid_on_eyes, position, eye_height, instance_name) in query.iter_mut() {
        let Some(instance) = instance_container.get(instance_name) else {
            continue;
        };

        let adjusted_eye_y = position.y + (**eye_height as f64) - 0.1111111119389534;
        let eye_block_pos = BlockPos::from(Vec3::new(position.x, adjusted_eye_y, position.z));
        let fluid_at_eye = instance
            .read()
            .get_fluid_state(&eye_block_pos)
            .unwrap_or_default();
        let fluid_cutoff_y = eye_block_pos.y as f64 + (fluid_at_eye.height as f64 / 16f64);
        if fluid_cutoff_y > adjusted_eye_y {
            **fluid_on_eyes = fluid_at_eye.fluid;
        } else {
            **fluid_on_eyes = azalea_registry::Fluid::Empty;
        }
    }
}

/// A component that lists all the local player entities that have this entity
/// loaded. If this is empty, the entity will be removed from the ECS.
#[derive(Component, Clone, Deref, DerefMut)]
pub struct LoadedBy(pub HashSet<Entity>);

pub fn clamp_look_direction(mut query: Query<&mut LookDirection>) {
    for mut look_direction in &mut query {
        look_direction.y_rot = look_direction.y_rot.rem_euclid(360.0);
        look_direction.x_rot = look_direction.x_rot.clamp(-90.0, 90.0) % 360.0;
    }
}

/// Sets the position of the entity. This doesn't update the cache in
/// azalea-world, and should only be used within azalea-world!
///
/// # Safety
/// Cached position in the world must be updated.
pub fn update_bounding_box(mut query: Query<(&Position, &mut Physics), Changed<Position>>) {
    for (position, mut physics) in query.iter_mut() {
        let bounding_box = physics.dimensions.make_bounding_box(position);
        physics.bounding_box = bounding_box;
    }
}

/// Marks an entity that's in a loaded chunk. This is updated at the beginning
/// of every tick.
#[derive(Component, Clone, Debug, Copy)]
pub struct InLoadedChunk;

/// Update the [`InLoadedChunk`] component for all entities in the world.
pub fn update_in_loaded_chunk(
    mut commands: bevy_ecs::system::Commands,
    query: Query<(Entity, &InstanceName, &Position)>,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, instance_name, position) in &query {
        let player_chunk_pos = ChunkPos::from(position);
        let Some(instance_lock) = instance_container.get(instance_name) else {
            continue;
        };

        let in_loaded_chunk = instance_lock.read().chunks.get(&player_chunk_pos).is_some();
        if in_loaded_chunk {
            commands.entity(entity).insert(InLoadedChunk);
        } else {
            commands.entity(entity).remove::<InLoadedChunk>();
        }
    }
}
