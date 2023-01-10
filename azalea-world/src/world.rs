use crate::{
    entity::{self, Entity, MinecraftEntityId, WorldName},
    ChunkStorage, EntityInfos, PartialChunkStorage, PartialEntityInfos, WorldContainer,
};
use azalea_core::ChunkPos;
use bevy_ecs::{
    component::Component,
    system::{Commands, Query},
};
use std::fmt::Formatter;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

/// PartialWorlds are usually owned by clients, and hold strong references to
/// chunks and entities in [`WeakWorld`]s.
///
/// Basically, they hold the chunks and entities that are within render
/// distance but can still access chunks and entities owned by other
/// `PartialWorld`s that have the same `WeakWorld`.
///
/// This is primarily useful for having multiple clients in the same world.
pub struct PartialWorld {
    pub chunks: PartialChunkStorage,
    /// Some metadata about entities, like what entities are in certain chunks.
    /// This does not contain the entity data itself, that's in the ECS.
    pub entity_infos: PartialEntityInfos,
}

impl PartialWorld {
    pub fn new(
        chunk_radius: u32,
        owner_entity: Option<Entity>,
        entity_infos: &mut EntityInfos,
    ) -> Self {
        PartialWorld {
            chunks: PartialChunkStorage::new(chunk_radius),
            entity_infos: PartialEntityInfos::new(owner_entity, entity_infos),
        }
    }

    /// Add an entity to the storage.
    #[inline]
    pub fn add_entity(
        &mut self,
        commands: &mut Commands,
        bundle: impl bevy_ecs::bundle::Bundle,
        entity_infos: &mut EntityInfos,
        world: &mut World,
        query: Query<(&entity::Position, &MinecraftEntityId, &entity::EntityUuid)>,
        id_query: Query<&MinecraftEntityId>,
    ) {
        let mut entity_commands = commands.spawn(bundle);
        let entity = entity_commands.id();
        let (position, &id, uuid) = query.get(entity).unwrap();
        let chunk_pos = ChunkPos::from(*position);

        // check every entity in this entitys chunk to make sure it doesn't already
        // exist there
        if let Some(entities_in_chunk) = world.entities_by_chunk.get(&chunk_pos) {
            for entity in entities_in_chunk {
                if id_query.get(*entity).unwrap() == &id {
                    // the entity is already in the world, so remove that extra entity we just made
                    entity_commands.despawn();
                    return;
                }
            }
        }

        let partial_entity_infos = &mut self.entity_infos;
        partial_entity_infos.loaded_entity_ids.insert(id);

        // add the entity to the indexes
        world
            .entities_by_chunk
            .entry(chunk_pos)
            .or_default()
            .insert(entity);
        entity_infos.entity_by_uuid.insert(**uuid, entity);
        // set our updates_received to the shared updates_received, unless it's
        // not there in which case set both to 1
        if let Some(&shared_updates_received) = entity_infos.updates_received.get(&entity) {
            // 0 means we're never tracking updates for this entity
            if shared_updates_received != 0 || Some(entity) == partial_entity_infos.owner_entity {
                partial_entity_infos
                    .updates_received
                    .insert(id, shared_updates_received);
            }
        } else {
            entity_infos.updates_received.insert(entity, 1);
            partial_entity_infos.updates_received.insert(id, 1);
        }
    }
}

/// A component marker signifying that the entity may have been removed from the
/// world, but we're not entirely sure.
#[derive(Component)]
pub struct MaybeRemovedEntity;

/// Clear all entities in a chunk. This will not clear them from the
/// shared storage unless there are no other references to them.
pub fn clear_entities_in_chunk(
    commands: &mut Commands,
    partial_entity_infos: &mut PartialEntityInfos,
    chunk: &ChunkPos,
    world_container: &WorldContainer,
    world_name: &WorldName,
    query: Query<&MinecraftEntityId>,
) {
    let world_lock = world_container.get(world_name).unwrap();
    let world = world_lock.read();

    if let Some(entities) = world.entities_by_chunk.get(chunk).cloned() {
        for &entity in &entities {
            let id = query.get(entity).unwrap();
            if partial_entity_infos.loaded_entity_ids.remove(id) {
                // maybe remove it from the storage
                commands.entity(entity).insert(MaybeRemovedEntity);
            }
        }
    }
}

/// A world where the chunks are stored as weak pointers. This is used for
/// shared worlds.
#[derive(Default, Debug)]
pub struct World {
    pub chunks: ChunkStorage,

    /// An index of all the entities we know are in the chunks of the world
    pub entities_by_chunk: HashMap<ChunkPos, HashSet<Entity>>,
}

impl Debug for PartialWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("chunk_storage", &self.chunks)
            .field("entity_storage", &self.entity_infos)
            .finish()
    }
}

impl Default for PartialWorld {
    /// Creates a completely self-contained `PartialWorld`. This is only for
    /// testing and shouldn't be used in actual code!
    fn default() -> Self {
        let chunk_storage = PartialChunkStorage::default();
        let entity_storage = PartialEntityInfos::default();
        Self {
            chunks: chunk_storage,
            entity_infos: entity_storage,
        }
    }
}
