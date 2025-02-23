//! Disconnect a client from the server.

use azalea_chat::FormattedText;
use azalea_entity::{EntityBundle, InLoadedChunk, LocalEntity, metadata::PlayerMetadataBundle};
use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::{EventReader, EventWriter},
    prelude::Event,
    query::{Changed, With},
    schedule::IntoSystemConfigs,
    system::{Commands, Query},
};
use derive_more::Deref;
use tracing::trace;

use crate::{
    InstanceHolder, client::JoinedClientBundle, events::LocalPlayerEvents,
    raw_connection::RawConnection,
};

pub struct DisconnectPlugin;
impl Plugin for DisconnectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DisconnectEvent>().add_systems(
            PostUpdate,
            (
                update_read_packets_task_running_component,
                disconnect_on_connection_dead,
                remove_components_from_disconnected_players,
            )
                .chain(),
        );
    }
}

/// An event sent when a client is getting disconnected.
#[derive(Event)]
pub struct DisconnectEvent {
    pub entity: Entity,
    pub reason: Option<FormattedText>,
}

/// A system that removes the several components from our clients when they get
/// a [`DisconnectEvent`].
pub fn remove_components_from_disconnected_players(
    mut commands: Commands,
    mut events: EventReader<DisconnectEvent>,
    mut loaded_by_query: Query<&mut azalea_entity::LoadedBy>,
) {
    for DisconnectEvent { entity, .. } in events.read() {
        trace!("Got DisconnectEvent for {entity:?}");
        commands
            .entity(*entity)
            .remove::<JoinedClientBundle>()
            .remove::<EntityBundle>()
            .remove::<InstanceHolder>()
            .remove::<PlayerMetadataBundle>()
            .remove::<InLoadedChunk>()
            // this makes it close the tcp connection
            .remove::<RawConnection>()
            // swarm detects when this tx gets dropped to fire SwarmEvent::Disconnect
            .remove::<LocalPlayerEvents>();
        // note that we don't remove the client from the ECS, so if they decide
        // to reconnect they'll keep their state

        // now we have to remove ourselves from the LoadedBy for every entity.
        // in theory this could be inefficient if we have massive swarms... but in
        // practice this is fine.
        for mut loaded_by in &mut loaded_by_query.iter_mut() {
            loaded_by.remove(entity);
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Deref)]
pub struct IsConnectionAlive(bool);

fn update_read_packets_task_running_component(
    query: Query<(Entity, &RawConnection)>,
    mut commands: Commands,
) {
    for (entity, raw_connection) in &query {
        let running = raw_connection.is_alive();
        commands.entity(entity).insert(IsConnectionAlive(running));
    }
}

#[allow(clippy::type_complexity)]
fn disconnect_on_connection_dead(
    query: Query<(Entity, &IsConnectionAlive), (Changed<IsConnectionAlive>, With<LocalEntity>)>,
    mut disconnect_events: EventWriter<DisconnectEvent>,
) {
    for (entity, &is_connection_alive) in &query {
        if !*is_connection_alive {
            disconnect_events.send(DisconnectEvent {
                entity,
                reason: None,
            });
        }
    }
}
