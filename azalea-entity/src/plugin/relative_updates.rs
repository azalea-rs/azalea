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

use std::sync::Arc;

use azalea_world::{MinecraftEntityId, PartialInstance};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use tracing::warn;

use crate::LocalEntity;

/// An [`EntityCommand`] that applies a "relative update" to an entity, which
/// means this update won't be run multiple times by different clients in the
/// same world.
///
/// This is used to avoid a bug where when there's multiple clients in the same
/// world and an entity sends a relative move packet to all clients, its
/// position gets desynced since the relative move is applied multiple times.
///
/// Don't use this unless you actually got an entity update packet that all
/// other clients within render distance will get too. You usually don't need
/// this when the change isn't relative either.
pub struct RelativeEntityUpdate {
    pub partial_world: Arc<RwLock<PartialInstance>>,
    // a function that takes the entity and updates it
    pub update: Box<dyn FnOnce(&mut EntityWorldMut) + Send + Sync>,
}
impl RelativeEntityUpdate {
    pub fn new(
        partial_world: Arc<RwLock<PartialInstance>>,
        update: impl FnOnce(&mut EntityWorldMut) + Send + Sync + 'static,
    ) -> Self {
        Self {
            partial_world,
            update: Box::new(update),
        }
    }
}

/// A component that counts the number of times this entity has been modified.
///
/// This is used for making sure two clients don't do the same relative update
/// on an entity.
///
/// If an entity is local (i.e. it's a client/LocalEntity), this component
/// should NOT be present in the entity.
#[derive(Component, Debug, Deref, DerefMut)]
pub struct UpdatesReceived(u32);

impl EntityCommand for RelativeEntityUpdate {
    fn apply(self, mut entity: EntityWorldMut) {
        let partial_entity_infos = &mut self.partial_world.write().entity_infos;

        if Some(entity.id()) == partial_entity_infos.owner_entity {
            // if the entity owns this partial world, it's always allowed to update itself
            (self.update)(&mut entity);
            return;
        };

        let entity_id = *entity.get::<MinecraftEntityId>().unwrap();
        if entity.contains::<LocalEntity>() {
            // a client tried to update another client, which isn't allowed
            return;
        }

        let this_client_updates_received = partial_entity_infos
            .updates_received
            .get(&entity_id)
            .copied();

        let can_update = if let Some(updates_received) = entity.get::<UpdatesReceived>() {
            this_client_updates_received.unwrap_or(1) == **updates_received
        } else {
            // no UpdatesReceived means the entity was just spawned
            true
        };
        if can_update {
            let new_updates_received = this_client_updates_received.unwrap_or(0) + 1;
            partial_entity_infos
                .updates_received
                .insert(entity_id, new_updates_received);

            entity.insert(UpdatesReceived(new_updates_received));

            (self.update)(&mut entity);
        }
    }
}

/// A system that logs a warning if an entity has both [`UpdatesReceived`]
/// and [`LocalEntity`].
pub fn debug_detect_updates_received_on_local_entities(
    query: Query<Entity, (With<LocalEntity>, With<UpdatesReceived>)>,
) {
    for entity in &query {
        warn!("Entity {entity:?} has both LocalEntity and UpdatesReceived");
    }
}
