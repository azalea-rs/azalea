use crate::entity::{self, Entity, EntityUuid, MinecraftEntityId, Position};
use azalea_core::{ChunkPos, ResourceLocation, Vec3};
use bevy_app::{App, CoreStage, Plugin};
use bevy_ecs::{
    query::{Changed, QueryEntityError, QueryState, ReadOnlyWorldQuery, WorldQuery},
    schedule::SystemSet,
    system::{Query, ResMut, Resource},
    world::{EntityMut, EntityRef},
};
use log::warn;
use nohash_hasher::{IntMap, IntSet};
use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::{Arc, Weak},
};
use uuid::Uuid;

/// Plugin handling some basic entity functionality.
pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Last,
            SystemSet::new().with_system(update_entity_chunk_positions),
        );
    }
}

// How entity updates are processed (to avoid issues with shared worlds)
// - each bot contains a map of { entity id: updates received }
// - the shared world also contains a canonical "true" updates received for each
//   entity
// - when a client loads an entity, its "updates received" is set to the same as
//   the global "updates received"
// - when the shared world sees an entity for the first time, the "updates
//   received" is set to 1.
// - clients can force the shared "updates received" to 0 to make it so certain
//   entities (i.e. other bots in our swarm) don't get confused and updated by
//   other bots
// - when a client gets an update to an entity, we check if our "updates
//   received" is the same as the shared world's "updates received": if it is,
//   then process the update and increment the client's and shared world's
//   "updates received" if not, then we simply increment our local "updates
//   received" and do nothing else

/// Store a map of entities by ID. To get an iterator over all entities, use
/// `storage.shared.read().entities` [`WeakEntityStorage::entities`].
///
/// This is meant to be used with shared worlds.
///
/// You can access the shared storage with `world.shared.read()`.
#[derive(Debug, Default)]
pub struct PartialEntityInfos {
    // note: using MinecraftEntityId for entity ids is acceptable here since there's no chance of
    // collisions here
    /// The entity id of the player that owns this partial world. This will
    /// make [`PartialWorld::entity_mut`] pretend the entity doesn't exist so
    /// it doesn't get modified from outside sources.
    ///
    /// [`PartialWorld::entity_mut`]: crate::PartialWorld::entity_mut
    pub owner_entity: Option<Entity>,
    /// A counter for each entity that tracks how many updates we've observed
    /// for it.
    ///
    /// This is used for shared worlds (i.e. swarms), to make sure we don't
    /// update entities twice on accident.
    pub updates_received: IntMap<MinecraftEntityId, u32>,
    /// A set of all the entity ids in render distance.
    pub(crate) loaded_entity_ids: IntSet<MinecraftEntityId>,
}

impl PartialEntityInfos {
    pub fn new(owner_entity: Option<Entity>, entity_infos: &mut EntityInfos) -> Self {
        if let Some(owner_entity) = owner_entity {
            entity_infos.updates_received.insert(owner_entity, 0);
        }
        Self {
            owner_entity,
            updates_received: IntMap::default(),
            loaded_entity_ids: IntSet::default(),
        }
    }

    /// Whether the entity with the given id is being loaded by this storage.
    /// If you want to check whether the entity is in the shared storage, use
    /// [`WeakEntityStorage::contains_id`].
    #[inline]
    pub fn limited_contains_id(&self, id: MinecraftEntityId) -> bool {
        self.loaded_entity_ids.contains(&id)
    }

    /// Get an [`Entity`] from the given [`MinecraftEntityId`] (which is just a
    /// u32 internally) if the entity is being loaded by this storage.
    #[inline]
    pub fn limited_get_by_id(
        &self,
        id: MinecraftEntityId,
        entity_infos: &mut EntityInfos,
    ) -> Option<Entity> {
        if self.limited_contains_id(id) {
            entity_infos
                .minecraft_entity_ids_to_azalea_entities
                .get(&id)
                .copied()
        } else {
            None
        }
    }

    /// Returns whether we're allowed to update this entity (to prevent two
    /// clients in a shared world updating it twice), and acknowleges that
    /// we WILL update it if it's true. Don't call this unless you actually
    /// got an entity update that all other clients within render distance
    /// will get too.
    pub fn maybe_update(&mut self, id: MinecraftEntityId, entity_infos: &mut EntityInfos) -> bool {
        let this_client_updates_received = self.updates_received.get(&id).copied();

        let entity = entity_infos
            .minecraft_entity_ids_to_azalea_entities
            .get(&id)
            .expect("entity should be in minecraft_entity_ids_to_azalea_entities")
            .clone();

        let shared_updates_received = entity_infos.updates_received.get(&entity).copied();

        let can_update = this_client_updates_received == shared_updates_received;
        if can_update {
            let new_updates_received = this_client_updates_received.unwrap_or(0) + 1;
            self.updates_received.insert(id, new_updates_received);
            entity_infos
                .updates_received
                .insert(entity, new_updates_received);
            true
        } else {
            false
        }
    }
}

/// This is useful so functions that return an IntSet of entity ids can return a
/// reference to nothing.
static EMPTY_ENTITY_ID_INTSET: Lazy<HashSet<Entity>> = Lazy::new(|| HashSet::default());

// TODO: optimization: switch out the `HashMap<Entity, _>`s for `IntMap`s

/// Things that are shared between all the partial worlds.
#[derive(Resource, Default)]
pub struct EntityInfos {
    // in WeakEntityInfos, we have to use [`Entity`] since there *is* a chance of collision if
    // we'd have used Minecraft entity IDs
    /// The number of `PartialWorld`s that have this entity loaded.
    /// (this is reference counting)
    pub(crate) entity_reference_count: HashMap<Entity, usize>,
    /// An index of all the entities we know are in a chunk
    pub(crate) entities_by_world_name_and_chunk:
        HashMap<ResourceLocation, HashMap<ChunkPos, HashSet<Entity>>>,
    /// An index of entities by their UUIDs
    pub(crate) entity_by_uuid: HashMap<Uuid, Entity>,

    /// The canonical number of updates we've gotten for every entity.
    pub updates_received: HashMap<Entity, u32>,

    /// The map of Minecraft entity ids to Azalea ECS entities.
    pub minecraft_entity_ids_to_azalea_entities: HashMap<u32, Entity>,
}

impl EntityInfos {
    pub fn new() -> Self {
        Self {
            entity_reference_count: HashMap::default(),
            entities_by_world_name_and_chunk: HashMap::default(),
            entity_by_uuid: HashMap::default(),
            updates_received: HashMap::default(),

            minecraft_entity_ids_to_azalea_entities: HashMap::default(),
        }
    }

    /// Call this if a [`PartialEntityStorage`] just removed an entity.
    ///
    /// It'll decrease the reference count and remove the entity from the
    /// storage if there's no more references to it.
    ///
    /// Returns whether the entity was removed.
    pub fn remove_entity_if_unused(&mut self, entity: Entity, uuid: Uuid, chunk: ChunkPos) -> bool {
        if let Some(count) = self.entity_reference_count.get_mut(&entity) {
            *count -= 1;
            if *count == 0 {
                self.entity_reference_count.remove(&entity);
                return true;
            }
        } else {
            warn!("Tried to remove entity but it was not found.");
            return false;
        }
        if self.entities_by_chunk.remove(&chunk).is_none() {
            warn!("Tried to remove entity from chunk {chunk:?} but it was not found.");
        }
        if self.entity_by_uuid.remove(&uuid).is_none() {
            warn!("Tried to remove entity from uuid {uuid:?} but it was not found.");
        }
        if self.updates_received.remove(&entity).is_none() {
            // if this happens it means we weren't tracking the updates_received for the
            // client (bad)
            warn!("Tried to remove entity from updates_received but it was not found.");
        }
        true
    }

    /// Remove a chunk from the storage if the entities in it have no strong
    /// references left.
    pub fn remove_chunk_if_unused(&mut self, chunk: &ChunkPos) {
        if let Some(entities) = self.entities_by_chunk.get(chunk) {
            if entities.is_empty() {
                self.entities_by_chunk.remove(chunk);
            }
        }
    }

    /// Whether the entity is in the shared storage. To check if a Minecraft
    /// entity ID is in the storage, you'll have to use
    /// [`PartialEntityInfo::limited_contains_id`].
    #[inline]
    pub fn contains_entity(&self, id: Entity) -> bool {
        self.entity_reference_count.contains_key(&id)
    }

    /// Returns a set of entities in the given chunk.
    pub fn entities_in_chunk<F>(&self, chunk: &ChunkPos) -> &HashSet<Entity> {
        self.entities_by_chunk
            .get(chunk)
            .unwrap_or(&EMPTY_ENTITY_ID_INTSET)
    }

    pub fn entity_by_uuid(&self, uuid: &Uuid) -> Option<&Entity> {
        self.entity_by_uuid.get(uuid)
    }
}

/// Update the chunk position indexes in [`EntityInfos`].
fn update_entity_chunk_positions(
    query: Query<
        (
            Entity,
            &entity::Position,
            &mut entity::LastPosition,
            &mut entity::Physics,
        ),
        Changed<entity::Position>,
    >,
    entity_infos: ResMut<EntityInfos>,
) {
    for (entity, pos, last_pos, mut physics) in query.iter_mut() {
        let old_chunk = ChunkPos::from(*last_pos);
        let new_chunk = ChunkPos::from(*pos);

        if old_chunk != new_chunk {
            // move the entity from the old chunk to the new one
            if let Some(entities) = entity_infos.entities_by_chunk.get_mut(&old_chunk) {
                entities.remove(&entity);
            }
            entity_infos
                .entities_by_chunk
                .entry(new_chunk)
                .or_default()
                .insert(entity);
        }
    }
}

impl Debug for EntityInfos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityInfos").finish()
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::entity::metadata;

//     use super::*;
//     use azalea_core::Vec3;

//     #[test]
//     fn test_store_entity() {
//         let mut storage = PartialEntityInfos::default();
//         assert!(storage.limited_get_by_id(0).is_none());
//         assert!(storage.shared.read().get_by_id(0).is_none());

//         let uuid = Uuid::from_u128(100);
//         storage.insert(
//             0,
//             EntityData::new(
//                 uuid,
//                 Vec3::default(),
//                 EntityMetadata::Player(metadata::Player::default()),
//             ),
//         );
//         assert_eq!(storage.limited_get_by_id(0).unwrap().uuid, uuid);
//         assert_eq!(storage.shared.read().get_by_id(0).unwrap().uuid, uuid);

//         storage.remove_by_id(0);
//         assert!(storage.limited_get_by_id(0).is_none());
//         assert!(storage.shared.read().get_by_id(0).is_none());
//     }
// }
