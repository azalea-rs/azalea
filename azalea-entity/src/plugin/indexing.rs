//! Stuff related to entity indexes and keeping track of entities in the world.

use azalea_core::ChunkPos;
use azalea_world::{InstanceContainer, InstanceName, MinecraftEntityId};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With, Without},
    system::{Commands, Query, Res, ResMut, Resource},
};
use log::{debug, error, info, warn};
use nohash_hasher::IntMap;
use std::{collections::HashMap, fmt::Debug};
use uuid::Uuid;

use crate::{EntityUuid, LastSentPosition, Local, Position};

use super::LoadedBy;

#[derive(Resource, Default)]
pub struct EntityUuidIndex {
    /// An index of entities by their UUIDs
    entity_by_uuid: HashMap<Uuid, Entity>,
}

/// An index of Minecraft entity IDs to Azalea ECS entities. This is a
/// `Component` so local players can keep track of entity IDs independently from
/// the instance.
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

/// Remove new entities that have the same id as an existing entity, and
/// increase the reference counts.
///
/// This is the reason why spawning entities into the ECS when you get a spawn
/// entity packet is okay. This system will make sure the new entity gets
/// combined into the old one.
#[allow(clippy::type_complexity)]
pub fn deduplicate_entities(
    mut commands: Commands,
    mut query: Query<
        (Entity, &MinecraftEntityId, &InstanceName),
        (Changed<MinecraftEntityId>, Without<Local>),
    >,
    mut loaded_by_query: Query<&mut LoadedBy>,
    mut entity_id_index_query: Query<&mut EntityIdIndex>,
    instance_container: Res<InstanceContainer>,
) {
    // if this entity already exists, remove it and keep the old one
    for (new_entity, id, world_name) in query.iter_mut() {
        if let Some(world_lock) = instance_container.get(world_name) {
            let world = world_lock.write();
            if let Some(old_entity) = world.entity_by_id.get(id) {
                if old_entity == &new_entity {
                    continue;
                }

                // this entity already exists!!! remove the one we just added but increase
                // the reference count
                let new_loaded_by = loaded_by_query
                    .get(new_entity)
                    .expect("Entities should always have the LoadedBy component ({new_entity:?} did not)")
                    .clone();

                // update the `EntityIdIndex`s of the local players that have this entity loaded
                for local_player in new_loaded_by.iter() {
                    let mut entity_id_index = entity_id_index_query
                        .get_mut(*local_player)
                        .expect("Local players should always have the EntityIdIndex component ({local_player:?} did not)");
                    entity_id_index.insert(*id, *old_entity);
                }

                let old_loaded_by = loaded_by_query.get_mut(*old_entity);
                // merge them if possible
                if let Ok(mut old_loaded_by) = old_loaded_by {
                    old_loaded_by.extend(new_loaded_by.iter());
                }
                commands.entity(new_entity).despawn();
                info!(
                    "Entity with id {id:?} / {new_entity:?} already existed in the world, merging it with {old_entity:?}"
                );
                break;
            }
        } else {
            error!("Entity was inserted into a world that doesn't exist.");
        }
    }
}

// when a local entity is added, if there was already an entity with the same id
// then delete the old entity
#[allow(clippy::type_complexity)]
pub fn deduplicate_local_entities(
    mut commands: Commands,
    mut query: Query<
        (Entity, &MinecraftEntityId, &InstanceName),
        (Changed<MinecraftEntityId>, With<Local>),
    >,
    instance_container: Res<InstanceContainer>,
) {
    // if this entity already exists, remove the old one
    for (new_entity, id, world_name) in query.iter_mut() {
        if let Some(world_lock) = instance_container.get(world_name) {
            let world = world_lock.write();
            if let Some(old_entity) = world.entity_by_id.get(id) {
                if old_entity == &new_entity {
                    // lol
                    continue;
                }

                commands.entity(*old_entity).despawn();
                debug!(
                    "Added local entity {id:?} / {new_entity:?} but already existed in world as {old_entity:?}, despawning {old_entity:?}"
                );
                break;
            }
        } else {
            error!("Entity was inserted into a world that doesn't exist.");
        }
    }
}

pub fn update_uuid_index(
    mut entity_infos: ResMut<EntityUuidIndex>,
    query: Query<(Entity, &EntityUuid, Option<&Local>), Changed<EntityUuid>>,
) {
    for (entity, &uuid, local) in query.iter() {
        // only add it if it doesn't already exist in
        // entity_infos.entity_by_uuid
        if local.is_none() {
            if let Some(old_entity) = entity_infos.entity_by_uuid.get(&uuid) {
                debug!(
                    "Entity with UUID {uuid:?} already existed in the world, not adding to index (old ecs id: {old_entity:?} / new ecs id: {entity:?})"
                );
                continue;
            }
        }
        entity_infos.entity_by_uuid.insert(*uuid, entity);
    }
}

/// System to keep the entity_by_id index up-to-date.
pub fn update_entity_by_id_index(
    mut query: Query<
        (Entity, &MinecraftEntityId, &InstanceName, Option<&Local>),
        Changed<MinecraftEntityId>,
    >,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, id, world_name, local) in query.iter_mut() {
        let world_lock = instance_container.get(world_name).unwrap();
        let mut world = world_lock.write();
        if local.is_none() {
            if let Some(old_entity) = world.entity_by_id.get(id) {
                debug!(
                    "Entity with ID {id:?} already existed in the world, not adding to index (old ecs id: {old_entity:?} / new ecs id: {entity:?})"
                );
                continue;
            }
        }
        world.entity_by_id.insert(*id, entity);
        debug!("Added {entity:?} to {world_name:?} with {id:?}.");
    }
}

/// Update the chunk position indexes in [`EntityUuidIndex`].
pub fn update_entity_chunk_positions(
    mut query: Query<(Entity, &Position, &mut LastSentPosition, &InstanceName), Changed<Position>>,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, pos, last_pos, world_name) in query.iter_mut() {
        let world_lock = instance_container.get(world_name).unwrap();
        let mut world = world_lock.write();

        let old_chunk = ChunkPos::from(*last_pos);
        let new_chunk = ChunkPos::from(*pos);

        if old_chunk != new_chunk {
            // move the entity from the old chunk to the new one
            if let Some(entities) = world.entities_by_chunk.get_mut(&old_chunk) {
                entities.remove(&entity);
            }
            world
                .entities_by_chunk
                .entry(new_chunk)
                .or_default()
                .insert(entity);
        }
    }
}

/// Despawn entities that aren't being loaded by anything.
#[allow(clippy::type_complexity)]
pub fn remove_despawned_entities_from_indexes(
    mut commands: Commands,
    mut entity_infos: ResMut<EntityUuidIndex>,
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
            if entity_infos.entity_by_uuid.remove(uuid).is_none() {
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
                warn!("Tried to remove entity from chunk {chunk:?} but the entity was not there.");
            }
        } else {
            warn!("Tried to remove entity from chunk {chunk:?} but the chunk was not found.");
        }
        // remove it from the uuid index
        if entity_infos.entity_by_uuid.remove(uuid).is_none() {
            warn!("Tried to remove entity {entity:?} from the uuid index but it was not there.");
        }
        if instance.entity_by_id.remove(minecraft_id).is_none() {
            warn!("Tried to remove entity {entity:?} from the id index but it was not there.");
        }
        // and now remove the entity from the ecs
        commands.entity(entity).despawn();
        debug!("Despawned entity {entity:?} because it was not loaded by anything.");
        return;
    }
}
