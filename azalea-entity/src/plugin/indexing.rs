//! Stuff related to entity indexes and keeping track of entities in the world.

use azalea_core::position::ChunkPos;
use azalea_world::{Instance, InstanceContainer, InstanceName, MinecraftEntityId};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Added, Changed},
    system::{Commands, Query, Res, ResMut, Resource},
};
use derive_more::{Deref, DerefMut};
use nohash_hasher::IntMap;
use std::{collections::HashMap, fmt::Debug};
use tracing::{debug, warn};
use uuid::Uuid;

use crate::{EntityUuid, Position};

use super::LoadedBy;

#[derive(Resource, Default)]
pub struct EntityUuidIndex {
    /// An index of entities by their UUIDs
    entity_by_uuid: HashMap<Uuid, Entity>,
}

/// An index of Minecraft entity IDs to Azalea ECS entities. This is a
/// `Component` so local players can keep track of entity IDs independently from
/// the instance.
///
/// If you need a per-instance instead of per-client version of this, you can
/// use [`Instance::entity_by_id`].
#[derive(Component, Default)]
pub struct EntityIdIndex {
    /// An index of entities by their MinecraftEntityId
    entity_by_id: IntMap<MinecraftEntityId, Entity>,
}

impl EntityUuidIndex {
    pub fn new() -> Self {
        Self {
            entity_by_uuid: HashMap::default(),
        }
    }

    pub fn get(&self, uuid: &Uuid) -> Option<Entity> {
        self.entity_by_uuid.get(uuid).copied()
    }

    pub fn contains_key(&self, uuid: &Uuid) -> bool {
        self.entity_by_uuid.contains_key(uuid)
    }

    pub fn insert(&mut self, uuid: Uuid, entity: Entity) {
        self.entity_by_uuid.insert(uuid, entity);
    }

    pub fn remove(&mut self, uuid: &Uuid) -> Option<Entity> {
        self.entity_by_uuid.remove(uuid)
    }
}

impl EntityIdIndex {
    pub fn get(&self, id: &MinecraftEntityId) -> Option<Entity> {
        self.entity_by_id.get(id).copied()
    }

    pub fn contains_key(&self, id: &MinecraftEntityId) -> bool {
        self.entity_by_id.contains_key(id)
    }

    pub fn insert(&mut self, id: MinecraftEntityId, entity: Entity) {
        self.entity_by_id.insert(id, entity);
    }

    pub fn remove(&mut self, id: &MinecraftEntityId) -> Option<Entity> {
        self.entity_by_id.remove(id)
    }
}

impl Debug for EntityUuidIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityUuidIndex").finish()
    }
}

/// The chunk position that an entity is currently in.
#[derive(Component, Debug, Deref, DerefMut)]
pub struct EntityChunkPos(pub ChunkPos);

/// Update the chunk position indexes in [`Instance::entities_by_chunk`].
///
/// [`Instance::entities_by_chunk`]: azalea_world::Instance::entities_by_chunk
pub fn update_entity_chunk_positions(
    mut query: Query<(Entity, &Position, &InstanceName, &mut EntityChunkPos), Changed<Position>>,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, pos, world_name, mut entity_chunk_pos) in query.iter_mut() {
        let instance_lock = instance_container.get(world_name).unwrap();
        let mut instance = instance_lock.write();

        let old_chunk = **entity_chunk_pos;
        let new_chunk = ChunkPos::from(*pos);
        if old_chunk != new_chunk {
            **entity_chunk_pos = new_chunk;

            if old_chunk != new_chunk {
                // move the entity from the old chunk to the new one
                if let Some(entities) = instance.entities_by_chunk.get_mut(&old_chunk) {
                    entities.remove(&entity);
                }
                instance
                    .entities_by_chunk
                    .entry(new_chunk)
                    .or_default()
                    .insert(entity);
            }
        }
    }
}

/// Insert new entities into [`Instance::entities_by_chunk`].
pub fn insert_entity_chunk_position(
    query: Query<(Entity, &Position, &InstanceName), Added<EntityChunkPos>>,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, pos, world_name) in query.iter() {
        let instance_lock = instance_container.get(world_name).unwrap();
        let mut instance = instance_lock.write();

        let chunk = ChunkPos::from(*pos);
        instance
            .entities_by_chunk
            .entry(chunk)
            .or_default()
            .insert(entity);
    }
}

/// Despawn entities that aren't being loaded by anything.
#[allow(clippy::type_complexity)]
pub fn remove_despawned_entities_from_indexes(
    mut commands: Commands,
    mut entity_uuid_index: ResMut<EntityUuidIndex>,
    instance_container: Res<InstanceContainer>,
    query: Query<
        (
            Entity,
            &EntityUuid,
            &MinecraftEntityId,
            &Position,
            &InstanceName,
            &LoadedBy,
        ),
        Changed<LoadedBy>,
    >,
) {
    for (entity, uuid, minecraft_id, position, world_name, loaded_by) in &query {
        let Some(instance_lock) = instance_container.get(world_name) else {
            // the instance isn't even loaded by us, so we can safely delete the entity
            debug!(
                "Despawned entity {entity:?} because it's in an instance that isn't loaded anymore"
            );
            if entity_uuid_index.entity_by_uuid.remove(uuid).is_none() {
                warn!(
                    "Tried to remove entity {entity:?} from the uuid index but it was not there."
                );
            }
            // and now remove the entity from the ecs
            commands.entity(entity).despawn();

            continue;
        };

        let mut instance = instance_lock.write();

        // if the entity has no references left, despawn it
        if !loaded_by.is_empty() {
            continue;
        }

        // remove the entity from the chunk index
        let chunk = ChunkPos::from(*position);
        if let Some(entities_in_chunk) = instance.entities_by_chunk.get_mut(&chunk) {
            if entities_in_chunk.remove(&entity) {
                // remove the chunk if there's no entities in it anymore
                if entities_in_chunk.is_empty() {
                    instance.entities_by_chunk.remove(&chunk);
                }
            } else {
                warn!(
                    "Tried to remove entity {entity:?} from chunk {chunk:?} but the entity was not there."
                );
            }
        } else {
            debug!("Tried to remove entity {entity:?} from chunk {chunk:?} but the chunk was not found.");
        }
        // remove it from the uuid index
        if entity_uuid_index.entity_by_uuid.remove(uuid).is_none() {
            warn!("Tried to remove entity {entity:?} from the uuid index but it was not there.");
        }
        if instance.entity_by_id.remove(minecraft_id).is_none() {
            warn!("Tried to remove entity {entity:?} from the id index but it was not there.");
        }
        // and now remove the entity from the ecs
        commands.entity(entity).despawn();
        debug!("Despawned entity {entity:?} because it was not loaded by anything.");
    }
}

pub fn add_entity_to_indexes(
    entity_id: MinecraftEntityId,
    ecs_entity: Entity,
    entity_uuid: Option<Uuid>,
    entity_id_index: &mut EntityIdIndex,
    entity_uuid_index: &mut EntityUuidIndex,
    instance: &mut Instance,
) {
    // per-client id index
    entity_id_index.insert(entity_id, ecs_entity);

    // per-instance id index
    instance.entity_by_id.insert(entity_id, ecs_entity);

    if let Some(uuid) = entity_uuid {
        // per-instance uuid index
        entity_uuid_index.insert(uuid, ecs_entity);
    }
}
