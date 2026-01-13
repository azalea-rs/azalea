//! Stuff related to entity indexes and keeping track of entities in the world.

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
};

use azalea_core::{entity_id::MinecraftEntityId, position::ChunkPos};
use azalea_world::{World, WorldName, Worlds};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};
use nohash_hasher::IntMap;
use tracing::{debug, trace, warn};
use uuid::Uuid;

use super::LoadedBy;
use crate::{EntityUuid, LocalEntity, Position};

#[derive(Default, Resource)]
pub struct EntityUuidIndex {
    /// An index of entities by their UUIDs
    entity_by_uuid: HashMap<Uuid, Entity>,
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

/// An index of Minecraft entity IDs to Azalea ECS entities.
///
/// This is a `Component` so local players can keep track of entity IDs
/// independently from the world.
///
/// If you need a per-world instead of per-client version of this, you can
/// use [`World::entity_by_id`].
#[derive(Component, Default)]
pub struct EntityIdIndex {
    /// An index of entities by their MinecraftEntityId
    entity_by_id: IntMap<MinecraftEntityId, Entity>,
    id_by_entity: HashMap<Entity, MinecraftEntityId>,
}

impl EntityIdIndex {
    pub fn get_by_minecraft_entity(&self, id: MinecraftEntityId) -> Option<Entity> {
        self.entity_by_id.get(&id).copied()
    }
    pub fn get_by_ecs_entity(&self, entity: Entity) -> Option<MinecraftEntityId> {
        self.id_by_entity.get(&entity).copied()
    }

    pub fn contains_minecraft_entity(&self, id: MinecraftEntityId) -> bool {
        self.entity_by_id.contains_key(&id)
    }
    pub fn contains_ecs_entity(&self, id: Entity) -> bool {
        self.id_by_entity.contains_key(&id)
    }

    pub fn insert(&mut self, id: MinecraftEntityId, entity: Entity) {
        self.entity_by_id.insert(id, entity);
        self.id_by_entity.insert(entity, id);
        trace!("Inserted {id} -> {entity:?} into a client's EntityIdIndex");
    }

    pub fn remove_by_minecraft_entity(&mut self, id: MinecraftEntityId) -> Option<Entity> {
        if let Some(entity) = self.entity_by_id.remove(&id) {
            trace!(
                "Removed {id} -> {entity:?} from a client's EntityIdIndex (using EntityIdIndex::remove)"
            );
            self.id_by_entity.remove(&entity);
            Some(entity)
        } else {
            trace!(
                "Failed to remove {id} from a client's EntityIdIndex (using EntityIdIndex::remove)"
            );
            None
        }
    }

    pub fn remove_by_ecs_entity(&mut self, entity: Entity) -> Option<MinecraftEntityId> {
        if let Some(id) = self.id_by_entity.remove(&entity) {
            trace!(
                "Removed {id} -> {entity:?} from a client's EntityIdIndex (using EntityIdIndex::remove_by_ecs_entity)."
            );
            self.entity_by_id.remove(&id);
            Some(id)
        } else {
            // this is expected to happen when despawning entities if it was already
            // despawned for another reason (like because the client received a
            // remove_entities packet, or if we're in a shared world where entity ids are
            // different for each client)
            trace!(
                "Failed to remove {entity:?} from a client's EntityIdIndex (using EntityIdIndex::remove_by_ecs_entity). This may be expected behavior."
            );
            None
        }
    }
}

impl Debug for EntityUuidIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EntityUuidIndex").finish()
    }
}

/// The chunk position that an entity is currently in.
#[derive(Component, Debug, Deref, DerefMut)]
pub struct EntityChunkPos(pub ChunkPos);

/// Update the chunk position indexes in [`World::entities_by_chunk`].
///
/// [`World::entities_by_chunk`]: azalea_world::World::entities_by_chunk
pub fn update_entity_chunk_positions(
    mut query: Query<(Entity, &Position, &WorldName, &mut EntityChunkPos), Changed<Position>>,
    worlds: Res<Worlds>,
) {
    for (entity, pos, world_name, mut entity_chunk_pos) in query.iter_mut() {
        let old_chunk = **entity_chunk_pos;
        let new_chunk = ChunkPos::from(*pos);
        if old_chunk != new_chunk {
            **entity_chunk_pos = new_chunk;

            if old_chunk != new_chunk {
                let Some(world_lock) = worlds.get(world_name) else {
                    continue;
                };
                let mut world = world_lock.write();

                // move the entity from the old chunk to the new one
                if let Some(entities) = world.entities_by_chunk.get_mut(&old_chunk) {
                    entities.remove(&entity);
                }
                world
                    .entities_by_chunk
                    .entry(new_chunk)
                    .or_default()
                    .insert(entity);
                trace!("Entity {entity:?} moved from {old_chunk:?} to {new_chunk:?}");
            }
        }
    }
}

/// Insert new entities into [`World::entities_by_chunk`].
pub fn insert_entity_chunk_position(
    query: Query<(Entity, &Position, &WorldName), Added<EntityChunkPos>>,
    worlds: Res<Worlds>,
) {
    for (entity, pos, world_name) in query.iter() {
        let Some(world_lock) = worlds.get(world_name) else {
            // entity must've been despawned already
            continue;
        };
        let mut world = world_lock.write();

        let chunk = ChunkPos::from(*pos);
        world
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
    worlds: Res<Worlds>,
    query: Query<
        (
            Entity,
            &EntityUuid,
            &MinecraftEntityId,
            &Position,
            &WorldName,
            &LoadedBy,
        ),
        (Changed<LoadedBy>, Without<LocalEntity>),
    >,
    mut entity_id_index_query: Query<&mut EntityIdIndex>,
) {
    for (entity, uuid, minecraft_id, position, world_name, loaded_by) in &query {
        let Some(world_lock) = worlds.get(world_name) else {
            // the world isn't even loaded by us, so we can safely delete the entity
            debug!("Despawned entity {entity:?} because it's in a world that isn't loaded anymore");
            if entity_uuid_index.entity_by_uuid.remove(uuid).is_none() {
                warn!(
                    "Tried to remove entity {entity:?} from the uuid index but it was not there."
                );
            }
            // and now remove the entity from the ecs
            commands.entity(entity).despawn();

            continue;
        };

        let mut world = world_lock.write();

        // if the entity is being loaded by any of our clients, don't despawn it
        if !loaded_by.is_empty() {
            continue;
        }

        // remove the entity from the chunk index
        let chunk = ChunkPos::from(position);
        match world.entities_by_chunk.get_mut(&chunk) {
            Some(entities_in_chunk) => {
                if entities_in_chunk.remove(&entity) {
                    // remove the chunk if there's no entities in it anymore
                    if entities_in_chunk.is_empty() {
                        world.entities_by_chunk.remove(&chunk);
                    }
                } else {
                    // search all the other chunks for it :(
                    let mut found_in_other_chunks = HashSet::new();
                    for (other_chunk, entities_in_other_chunk) in &mut world.entities_by_chunk {
                        if entities_in_other_chunk.remove(&entity) {
                            found_in_other_chunks.insert(other_chunk);
                        }
                    }
                    if found_in_other_chunks.is_empty() {
                        warn!(
                            "Tried to remove entity {entity:?} from chunk {chunk:?} but the entity was not there or in any other chunks."
                        );
                    } else {
                        warn!(
                            "Tried to remove entity {entity:?} from chunk {chunk:?} but the entity was not there. Found in and removed from other chunk(s): {found_in_other_chunks:?}"
                        );
                    }
                }
            }
            _ => {
                let mut found_in_other_chunks = HashSet::new();
                for (other_chunk, entities_in_other_chunk) in &mut world.entities_by_chunk {
                    if entities_in_other_chunk.remove(&entity) {
                        found_in_other_chunks.insert(other_chunk);
                    }
                }
                if found_in_other_chunks.is_empty() {
                    warn!(
                        "Tried to remove entity {entity:?} from chunk {chunk:?} but the chunk was not found and the entity wasn't in any other chunks."
                    );
                } else {
                    warn!(
                        "Tried to remove entity {entity:?} from chunk {chunk:?} but the chunk was not found. Entity found in and removed from other chunk(s): {found_in_other_chunks:?}"
                    );
                }
            }
        }
        // remove it from the uuid index
        if entity_uuid_index.entity_by_uuid.remove(uuid).is_none() {
            warn!("Tried to remove entity {entity:?} from the uuid index but it was not there.");
        }
        if world.entity_by_id.remove(minecraft_id).is_none() {
            debug!(
                "Tried to remove entity {entity:?} from the per-world entity id index but it was not there. This may be expected if you're in a shared world."
            );
        }

        // remove it from every client's EntityIdIndex
        for mut entity_id_index in entity_id_index_query.iter_mut() {
            entity_id_index.remove_by_ecs_entity(entity);
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
    world: &mut World,
) {
    // per-client id index
    entity_id_index.insert(entity_id, ecs_entity);

    // per-world id index
    world.entity_by_id.insert(entity_id, ecs_entity);

    if let Some(uuid) = entity_uuid {
        // per-world uuid index
        entity_uuid_index.insert(uuid, ecs_entity);
    }
}
