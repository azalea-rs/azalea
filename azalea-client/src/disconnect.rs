//! Disconnect a client from the server.

use azalea_ecs::{
    app::{App, CoreStage, Plugin},
    entity::Entity,
    event::EventReader,
    system::Commands,
};

use crate::client::JoinedClientBundle;

pub struct DisconnectPlugin;
impl Plugin for DisconnectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DisconnectEvent>()
            .add_system_to_stage(CoreStage::PostUpdate, handle_disconnect);
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
