use azalea_core::tick::GameTick;
use azalea_entity::{InLoadedChunk, LocalEntity};
use azalea_physics::PhysicsSet;
use azalea_protocol::packets::game::ServerboundPlayerLoaded;
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;

use crate::{mining::MiningSet, packet::game::SendPacketEvent};

pub struct PlayerLoadedPlugin;
impl Plugin for PlayerLoadedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GameTick,
            player_loaded_packet
                .after(PhysicsSet)
                .after(MiningSet)
                .after(crate::movement::send_position),
        );
    }
}

// this component is removed on respawn or disconnect
// (notably, it's not removed on login)

// mojmap interchangably calls it 'has client loaded' and 'has player loaded', i
// prefer the client one because it makes it clear that the component is only
// present on our own clients

#[derive(Component)]
pub struct HasClientLoaded;
#[allow(clippy::type_complexity)]
pub fn player_loaded_packet(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            With<LocalEntity>,
            With<InLoadedChunk>,
            Without<HasClientLoaded>,
        ),
    >,
) {
    for entity in query.iter() {
        commands.trigger(SendPacketEvent::new(entity, ServerboundPlayerLoaded));
        commands.entity(entity).insert(HasClientLoaded);
    }
}
