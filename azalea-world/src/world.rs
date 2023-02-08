use crate::{
    entity::{
        EntityInfos, EntityUuid, LoadedBy, Local, MinecraftEntityId, PartialEntityInfos, WorldName,
    },
    ChunkStorage, PartialChunkStorage, WorldContainer,
};
use azalea_core::ChunkPos;
use azalea_ecs::{
    entity::Entity,
    query::{Changed, With, Without},
    system::{Commands, Query, Res, ResMut},
};
use log::{debug, error, info};
use nohash_hasher::IntMap;
use std::fmt::Formatter;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

/// PartialWorlds are usually owned by clients, and hold strong references to
/// chunks and entities in [`World`]s.
///
/// Basically, they hold the chunks and entities that are within render
/// distance but can still access chunks and entities owned by other
/// `PartialWorld`s that have the same `World`.
///
/// This is primarily useful for having multiple clients in the same world.
pub struct PartialWorld {
    pub chunks: PartialChunkStorage,
    /// Some metadata about entities, like what entities are in certain chunks.
    /// This does not contain the entity data itself, that's in the ECS.
    pub entity_infos: PartialEntityInfos,
}

impl PartialWorld {
    pub fn new(chunk_radius: u32, owner_entity: Option<Entity>) -> Self {
        PartialWorld {
            chunks: PartialChunkStorage::new(chunk_radius),
            entity_infos: PartialEntityInfos::new(owner_entity),
        }
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
        (Entity, &MinecraftEntityId, &WorldName),
        (Changed<MinecraftEntityId>, Without<Local>),
    >,
    mut loaded_by_query: Query<&mut LoadedBy>,
    world_container: Res<WorldContainer>,
) {
    // if this entity already exists, remove it
    for (new_entity, id, world_name) in query.iter_mut() {
        if let Some(world_lock) = world_container.get(world_name) {
            let world = world_lock.write();
            if let Some(old_entity) = world.entity_by_id.get(id) {
                if old_entity == &new_entity {
                    continue;
                }

                // this entity already exists!!! remove the one we just added but increase
                // the reference count
                let new_loaded_by = loaded_by_query
                    .get(new_entity)
                    .unwrap_or_else(|_| panic!(
                        "Entities should always have the LoadedBy component ({new_entity:?} did not)"
                    ))
                    .clone();
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
        (Entity, &MinecraftEntityId, &WorldName),
        (Changed<MinecraftEntityId>, With<Local>),
    >,
    world_container: Res<WorldContainer>,
) {
    // if this entity already exists, remove the old one
    for (new_entity, id, world_name) in query.iter_mut() {
        if let Some(world_lock) = world_container.get(world_name) {
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
    mut entity_infos: ResMut<EntityInfos>,
    query: Query<(Entity, &EntityUuid), Changed<EntityUuid>>,
) {
    for (entity, &uuid) in query.iter() {
        // only add it if it doesn't already exist in
        // entity_infos.entity_by_uuid
        // if entity_infos.entity_by_uuid.contains_key(&uuid) {
        //     warn!("Entity with UUID {uuid:?} already existed in the world, not adding
        // to index (ecs id: {entity:?})", uuid=*uuid);     continue;
        // }
        entity_infos.entity_by_uuid.insert(*uuid, entity);
    }
}

// /// Clear all entities in a chunk. This will not clear them from the
// /// shared storage unless there are no other references to them.
// pub fn clear_entities_in_chunk(
//     mut commands: Commands,
//     partial_entity_infos: &mut PartialEntityInfos,
//     chunk: &ChunkPos,
//     world_container: &WorldContainer,
//     world_name: &WorldName,
//     mut query: Query<(&MinecraftEntityId, &mut ReferenceCount)>,
// ) {
//     let world_lock = world_container.get(world_name).unwrap();
//     let world = world_lock.read();

//     if let Some(entities) = world.entities_by_chunk.get(chunk).cloned() {
//         for &entity in &entities {
//             let (id, mut reference_count) = query.get_mut(entity).unwrap();
//             if partial_entity_infos.loaded_entity_ids.remove(id) {
//                 // decrease the reference count
//                 **reference_count -= 1;
//             }
//         }
//     }
// }

/// A world where the chunks are stored as weak pointers. This is used for
/// shared worlds.
#[derive(Default, Debug)]
pub struct World {
    pub chunks: ChunkStorage,

    /// An index of all the entities we know are in the chunks of the world
    pub entities_by_chunk: HashMap<ChunkPos, HashSet<Entity>>,

    /// An index of Minecraft entity IDs to Azalea ECS entities.
    pub entity_by_id: IntMap<MinecraftEntityId, Entity>,
}

impl World {
    /// Get an ECS [`Entity`] from a Minecraft entity ID.
    pub fn entity_by_id(&self, entity_id: &MinecraftEntityId) -> Option<Entity> {
        self.entity_by_id.get(entity_id).copied()
    }
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

/// System to keep the entity_by_id index up-to-date.
pub fn update_entity_by_id_index(
    mut query: Query<(Entity, &MinecraftEntityId, &WorldName), Changed<MinecraftEntityId>>,
    world_container: Res<WorldContainer>,
) {
    for (entity, id, world_name) in query.iter_mut() {
        let world_lock = world_container.get(world_name).unwrap();
        let mut world = world_lock.write();
        // if let Some(old_entity) = world.entity_by_id.get(id) {
        //     warn!(
        //         "Entity with ID {id:?} already existed in the world, not adding to
        // index (old ecs id: {old_entity:?} / new ecs id: {entity:?})"     );
        //     continue;
        // }
        world.entity_by_id.insert(*id, entity);
        debug!("Added {entity:?} to {world_name:?} with {id:?}.");
    }
}

impl From<ChunkStorage> for World {
    /// Make an empty world from this `ChunkStorage`. This is meant to be a
    /// convenience function for tests.
    fn from(chunks: ChunkStorage) -> Self {
        Self {
            chunks,
            entities_by_chunk: HashMap::new(),
            entity_by_id: IntMap::default(),
        }
    }
}
