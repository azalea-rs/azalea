//! Disconnect a client from the server.

use azalea_ecs::{
    app::{App, CoreStage, Plugin},
    entity::Entity,
    event::{EventReader, EventWriter},
    system::{Commands, Query},
    AppTickExt,
};

use crate::{client::JoinedClientBundle, LocalPlayer};

pub struct DisconnectPlugin;
impl Plugin for DisconnectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DisconnectEvent>()
            .add_system_to_stage(CoreStage::PostUpdate, handle_disconnect)
            .add_tick_system(disconnect_on_read_packets_ended);
    }
}

/// An event sent when a client is getting disconnected.
pub struct DisconnectEvent {
    pub entity: Entity,
}

/// System that removes the [`JoinedClientBundle`] from the entity when it
/// receives a [`DisconnectEvent`].
pub fn handle_disconnect(mut commands: Commands, mut events: EventReader<DisconnectEvent>) {
    for DisconnectEvent { entity } in events.iter() {
        commands.entity(*entity).remove::<JoinedClientBundle>();
    }
}

fn disconnect_on_read_packets_ended(
    local_player: Query<(Entity, &LocalPlayer)>,
    mut disconnect_events: EventWriter<DisconnectEvent>,
) {
    for (entity, local_player) in &local_player {
        if local_player.read_packets_task.is_finished() {
            disconnect_events.send(DisconnectEvent { entity });
        }
    }
}
