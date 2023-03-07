//! Implement things relating to entity datas, like an index of uuids to
//! entities.

use crate::{
    deduplicate_entities, deduplicate_local_entities,
    entity::{
        self, add_dead, update_bounding_box, EntityUuid, MinecraftEntityId, Position, WorldName,
    },
    update_entity_by_id_index, update_uuid_index, PartialWorld, WorldContainer,
};
use azalea_core::ChunkPos;
use bevy_app::{App, CoreSet, Plugin};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Added, Changed, With, Without},
    schedule::{IntoSystemConfig, IntoSystemConfigs, SystemSet},
    system::{Commands, EntityCommand, Query, Res, ResMut, Resource},
    world::{EntityMut, World},
};
use derive_more::{Deref, DerefMut};
use log::{debug, warn};
use nohash_hasher::IntMap;
use parking_lot::RwLock;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::Arc,
};
use uuid::Uuid;

use super::Local;

/// A Bevy [`SystemSet`] for various types of entity updates.
#[derive(SystemSet, Debug, Hash, Eq, PartialEq, Clone)]
pub enum EntityUpdateSet {
    /// Remove ECS entities that refer to an entity that was already in the ECS
    /// before.
    Deduplicate,
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
        app.add_system(
            remove_despawned_entities_from_indexes
                .in_base_set(CoreSet::PreUpdate)
                .in_set(EntityUpdateSet::Deindex),
        )
        .add_systems(
            (deduplicate_entities, deduplicate_local_entities)
                .in_base_set(CoreSet::PostUpdate)
                .in_set(EntityUpdateSet::Deduplicate),
        )
        .add_systems(
            (
                update_entity_chunk_positions,
                update_uuid_index,
                update_entity_by_id_index,
            )
                .in_set(EntityUpdateSet::Index),
        )
        .add_systems((
            add_updates_received,
            debug_new_entity,
            debug_detect_updates_received_on_local_entities,
            add_dead,
            update_bounding_box,
        ))
        .init_resource::<EntityInfos>();
    }
}

fn debug_new_entity(query: Query<(Entity, Option<&Local>), Added<MinecraftEntityId>>) {
    for (entity, local) in query.iter() {
        if local.is_some() {
            debug!("new local entity: {:?}", entity);
        } else {
            debug!("new entity: {:?}", entity);
        }
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

/// Keep track of certain metadatas that are only relevant for this partial
/// world.
#[derive(Debug, Default)]
pub struct PartialEntityInfos {
    // note: using MinecraftEntityId for entity ids is acceptable here since
    // there's no chance of collisions here
    /// The entity id of the player that owns this partial world. This will
    /// make [`RelativeEntityUpdate`] pretend the entity doesn't exist so
    /// it doesn't get modified from outside sources.
    pub owner_entity: Option<Entity>,
    /// A counter for each entity that tracks how many updates we've observed
    /// for it.
    ///
    /// This is used for shared worlds (i.e. swarms), to make sure we don't
    /// update entities twice on accident.
    pub updates_received: IntMap<MinecraftEntityId, u32>,
}

impl PartialEntityInfos {
    pub fn new(owner_entity: Option<Entity>) -> Self {
        Self {
            owner_entity,
            updates_received: IntMap::default(),
        }
    }
}

/// A [`Command`] that applies a "relative update" to an entity, which means
/// this update won't be run multiple times by different clients in the same
/// world.
///
/// This is used to avoid a bug where when there's multiple clients in the same
/// world and an entity sends a relative move packet to all clients, its
/// position gets desynced since the relative move is applied multiple times.
///
/// Don't use this unless you actually got an entity update packet that all
/// other clients within render distance will get too. You usually don't need
/// this when the change isn't relative either.
pub struct RelativeEntityUpdate {
    pub partial_world: Arc<RwLock<PartialWorld>>,
    // a function that takes the entity and updates it
    pub update: Box<dyn FnOnce(&mut EntityMut) + Send + Sync>,
}
impl EntityCommand for RelativeEntityUpdate {
    fn write(self, entity: Entity, world: &mut World) {
        let partial_entity_infos = &mut self.partial_world.write().entity_infos;

        let mut entity_mut = world.entity_mut(entity);

        if Some(entity) == partial_entity_infos.owner_entity {
            // if the entity owns this partial world, it's always allowed to update itself
            (self.update)(&mut entity_mut);
            return;
        };

        let entity_id = *entity_mut.get::<MinecraftEntityId>().unwrap();
        let Some(updates_received) = entity_mut.get_mut::<UpdatesReceived>() else {
            // a client tried to update another client, which isn't allowed
            return;
        };

        let this_client_updates_received = partial_entity_infos
            .updates_received
            .get(&entity_id)
            .copied();

        let can_update = this_client_updates_received.unwrap_or(1) == **updates_received;
        if can_update {
            let new_updates_received = this_client_updates_received.unwrap_or(0) + 1;
            partial_entity_infos
                .updates_received
                .insert(entity_id, new_updates_received);

            **entity_mut.get_mut::<UpdatesReceived>().unwrap() = new_updates_received;

            let mut entity = world.entity_mut(entity);
            (self.update)(&mut entity);
        }
    }
}

/// Things that are shared between all the partial worlds.
#[derive(Resource, Default)]
pub struct EntityInfos {
    /// An index of entities by their UUIDs
    pub(crate) entity_by_uuid: HashMap<Uuid, Entity>,
}

impl EntityInfos {
    pub fn new() -> Self {
        Self {
            entity_by_uuid: HashMap::default(),
        }
    }

    pub fn get_entity_by_uuid(&self, uuid: &Uuid) -> Option<Entity> {
        self.entity_by_uuid.get(uuid).copied()
    }
}

/// Update the chunk position indexes in [`EntityInfos`].
fn update_entity_chunk_positions(
    mut query: Query<
        (
            Entity,
            &entity::Position,
            &mut entity::LastSentPosition,
            &entity::WorldName,
        ),
        Changed<entity::Position>,
    >,
    world_container: Res<WorldContainer>,
) {
    for (entity, pos, last_pos, world_name) in query.iter_mut() {
        let world_lock = world_container.get(world_name).unwrap();
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
/// A component that lists all the local player entities that have this entity
/// loaded. If this is empty, the entity will be removed from the ECS.
#[derive(Component, Clone, Deref, DerefMut)]
pub struct LoadedBy(pub HashSet<Entity>);

/// A component that counts the number of times this entity has been modified.
/// This is used for making sure two clients don't do the same relative update
/// on an entity.
///
/// If an entity is local (i.e. it's a client/localplayer), this component
/// should NOT be present in the entity.
#[derive(Component, Debug, Deref, DerefMut)]
pub struct UpdatesReceived(u32);

#[allow(clippy::type_complexity)]
pub fn add_updates_received(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            Changed<MinecraftEntityId>,
            (Without<UpdatesReceived>, Without<Local>),
        ),
    >,
) {
    for entity in query.iter() {
        // entities always start with 1 update received
        commands.entity(entity).insert(UpdatesReceived(1));
    }
}

/// The [`UpdatesReceived`] component should never be on [`Local`] entities.
/// This warns if an entity has both components.
fn debug_detect_updates_received_on_local_entities(
    query: Query<Entity, (With<Local>, With<UpdatesReceived>)>,
) {
    for entity in &query {
        warn!("Entity {:?} has both Local and UpdatesReceived", entity);
    }
}

/// Despawn entities that aren't being loaded by anything.
fn remove_despawned_entities_from_indexes(
    mut commands: Commands,
    mut entity_infos: ResMut<EntityInfos>,
    world_container: Res<WorldContainer>,
    query: Query<(Entity, &EntityUuid, &Position, &WorldName, &LoadedBy), Changed<LoadedBy>>,
) {
    for (entity, uuid, position, world_name, loaded_by) in &query {
        let world_lock = world_container.get(world_name).unwrap();
        let mut world = world_lock.write();

        // if the entity has no references left, despawn it
        if !loaded_by.is_empty() {
            continue;
        }

        // remove the entity from the chunk index
        let chunk = ChunkPos::from(*position);
        if let Some(entities_in_chunk) = world.entities_by_chunk.get_mut(&chunk) {
            if entities_in_chunk.remove(&entity) {
                // remove the chunk if there's no entities in it anymore
                if entities_in_chunk.is_empty() {
                    world.entities_by_chunk.remove(&chunk);
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
        // and now remove the entity from the ecs
        commands.entity(entity).despawn();
        debug!("Despawned entity {entity:?} because it was not loaded by anything.");
        return;
    }
}

impl Debug for EntityInfos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityInfos").finish()
    }
}
