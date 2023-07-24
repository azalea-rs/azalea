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
use bevy_ecs::{
    prelude::{Component, Entity},
    query::{Changed, With, Without},
    system::{Commands, EntityCommand, Query},
    world::{EntityMut, World},
};
use derive_more::{Deref, DerefMut};
use log::warn;
use parking_lot::RwLock;

use crate::Local;

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
    pub update: Box<dyn FnOnce(&mut EntityMut) + Send + Sync>,
}

/// A component that counts the number of times this entity has been modified.
/// This is used for making sure two clients don't do the same relative update
/// on an entity.
///
/// If an entity is local (i.e. it's a client/localplayer), this component
/// should NOT be present in the entity.
#[derive(Component, Debug, Deref, DerefMut)]
pub struct UpdatesReceived(u32);

impl EntityCommand for RelativeEntityUpdate {
    fn apply(self, entity: Entity, world: &mut World) {
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
pub fn debug_detect_updates_received_on_local_entities(
    query: Query<Entity, (With<Local>, With<UpdatesReceived>)>,
) {
    for entity in &query {
        warn!("Entity {:?} has both Local and UpdatesReceived", entity);
    }
}
