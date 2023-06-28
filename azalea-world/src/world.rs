use crate::{
    entity::{
        EntityInfos, EntityUuid, InstanceName, LoadedBy, Local, MinecraftEntityId,
        PartialEntityInfos,
    },
    iterators::ChunkIterator,
    palette::Palette,
    ChunkStorage, InstanceContainer, PartialChunkStorage,
};
use azalea_block::{BlockState, BlockStates};
use azalea_core::{BlockPos, ChunkPos};
use bevy_ecs::{
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

/// PartialInstances are usually owned by clients, and hold strong references to
/// chunks and entities in [`Instance`]s.
///
/// Basically, they hold the chunks and entities that are within render
/// distance but can still access chunks and entities owned by other
/// `PartialInstance`s that have the same `Instance`.
///
/// This is primarily useful for having multiple clients in the same Instance.
pub struct PartialInstance {
    pub chunks: PartialChunkStorage,
    /// Some metadata about entities, like what entities are in certain chunks.
    /// This does not contain the entity data itself, that's in the ECS.
    pub entity_infos: PartialEntityInfos,
}

impl PartialInstance {
    pub fn new(chunk_radius: u32, owner_entity: Option<Entity>) -> Self {
        PartialInstance {
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
        (Entity, &MinecraftEntityId, &InstanceName),
        (Changed<MinecraftEntityId>, Without<Local>),
    >,
    mut loaded_by_query: Query<&mut LoadedBy>,
    instance_container: Res<InstanceContainer>,
) {
    // if this entity already exists, remove it
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
    mut entity_infos: ResMut<EntityInfos>,
    query: Query<(Entity, &EntityUuid, Option<&Local>), Changed<EntityUuid>>,
) {
    for (entity, &uuid, local) in query.iter() {
        // only add it if it doesn't already exist in
        // entity_infos.entity_by_uuid
        if local.is_none() {
            if let Some(old_entity) = entity_infos.entity_by_uuid.get(&uuid) {
                debug!(
                    "Entity with UUID {uuid:?} already existed in the world, not adding to
        index (old ecs id: {old_entity:?} / new ecs id: {entity:?})"
                );
                continue;
            }
        }
        entity_infos.entity_by_uuid.insert(*uuid, entity);
    }
}

// /// Clear all entities in a chunk. This will not clear them from the
// /// shared storage unless there are no other references to them.
// pub fn clear_entities_in_chunk(
//     mut commands: Commands,
//     partial_entity_infos: &mut PartialEntityInfos,
//     chunk: &ChunkPos,
//     instance_container: &WorldContainer,
//     world_name: &WorldName,
//     mut query: Query<(&MinecraftEntityId, &mut ReferenceCount)>,
// ) { let world_lock = instance_container.get(world_name).unwrap(); let world =
//   world_lock.read();

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
pub struct Instance {
    pub chunks: ChunkStorage,

    /// An index of all the entities we know are in the chunks of the world
    pub entities_by_chunk: HashMap<ChunkPos, HashSet<Entity>>,

    /// An index of Minecraft entity IDs to Azalea ECS entities.
    pub entity_by_id: IntMap<MinecraftEntityId, Entity>,
}

impl Instance {
    /// Get an ECS [`Entity`] from a Minecraft entity ID.
    pub fn entity_by_id(&self, entity_id: &MinecraftEntityId) -> Option<Entity> {
        self.entity_by_id.get(entity_id).copied()
    }

    pub fn get_block_state(&self, pos: &BlockPos) -> Option<BlockState> {
        self.chunks.get_block_state(pos)
    }

    pub fn set_block_state(&self, pos: &BlockPos, state: BlockState) -> Option<BlockState> {
        self.chunks.set_block_state(pos, state)
    }

    /// Find the coordinates of a block in the world.
    ///
    /// Note that this is sorted by `x+y+z` and not `x^2+y^2+z^2`, for
    /// optimization purposes.
    ///
    /// ```
    /// # fn example(client: &azalea_client::Client) {
    /// client.world().read().find_block(client.position(), &azalea_registry::Block::Chest.into());
    /// # }
    /// ```
    pub fn find_block(
        &self,
        nearest_to: impl Into<BlockPos>,
        block_states: &BlockStates,
    ) -> Option<BlockPos> {
        // iterate over every chunk in a 3d spiral pattern
        // and then check the palette for the block state

        let nearest_to: BlockPos = nearest_to.into();
        let start_chunk: ChunkPos = (&nearest_to).into();
        let iter = ChunkIterator::new(start_chunk, 32);

        for chunk_pos in iter {
            let chunk = self.chunks.get(&chunk_pos).unwrap();

            let mut nearest_found_pos: Option<BlockPos> = None;
            let mut nearest_found_distance = 0;

            for (section_index, section) in chunk.read().sections.iter().enumerate() {
                let maybe_has_block = match &section.states.palette {
                    Palette::SingleValue(id) => block_states.contains(&BlockState { id: *id }),
                    Palette::Linear(ids) => ids
                        .iter()
                        .any(|&id| block_states.contains(&BlockState { id })),
                    Palette::Hashmap(ids) => ids
                        .iter()
                        .any(|&id| block_states.contains(&BlockState { id })),
                    Palette::Global => true,
                };
                if !maybe_has_block {
                    continue;
                }

                for i in 0..4096 {
                    let block_state = section.states.get_at_index(i);
                    let block_state = BlockState { id: block_state };

                    if block_states.contains(&block_state) {
                        let (section_x, section_y, section_z) = section.states.coords_from_index(i);
                        let (x, y, z) = (
                            chunk_pos.x * 16 + (section_x as i32),
                            self.chunks.min_y + (section_index * 16) as i32 + section_y as i32,
                            chunk_pos.z * 16 + (section_z as i32),
                        );
                        let this_block_pos = BlockPos { x, y, z };
                        let this_block_distance = (nearest_to - this_block_pos).length_manhattan();
                        // only update if it's closer
                        if nearest_found_pos.is_none()
                            || this_block_distance < nearest_found_distance
                        {
                            nearest_found_pos = Some(this_block_pos);
                            nearest_found_distance = this_block_distance;
                        }
                    }
                }
            }

            // if we found the position, return it
            if nearest_found_pos.is_some() {
                return nearest_found_pos;
            }
        }

        None
    }
}

impl Debug for PartialInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("chunk_storage", &self.chunks)
            .field("entity_storage", &self.entity_infos)
            .finish()
    }
}

impl Default for PartialInstance {
    /// Creates a completely self-contained `PartialInstance`. This is only for
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
                    "Entity with ID {id:?} already existed in the world, not adding to
        index (old ecs id: {old_entity:?} / new ecs id: {entity:?})"
                );
                continue;
            }
        }
        world.entity_by_id.insert(*id, entity);
        debug!("Added {entity:?} to {world_name:?} with {id:?}.");
    }
}

impl From<ChunkStorage> for Instance {
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
