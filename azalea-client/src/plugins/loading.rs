use azalea_core::tick::GameTick;
use azalea_entity::{HasClientLoaded, InLoadedChunk, LocalEntity, update_in_loaded_chunk};
use azalea_physics::PhysicsSystems;
use azalea_protocol::packets::game::ServerboundPlayerLoaded;
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;

use crate::{mining::MiningSystems, packet::game::SendGamePacketEvent};

pub struct PlayerLoadedPlugin;
impl Plugin for PlayerLoadedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GameTick,
            // vanilla runs this on gameMode.tick()
            player_loaded_packet
                .after(update_in_loaded_chunk)
                .before(PhysicsSystems)
                .before(MiningSystems)
                .before(crate::movement::send_position),
        );
    }
}

// this component is removed on respawn or disconnect
// (notably, it's not removed on login)

// mojmap interchangeably calls it 'has client loaded' and 'has player loaded',
// i prefer the client one because it makes it clear that the component is only
// present on our own clients

#[allow(clippy::type_complexity)]
pub fn player_loaded_packet(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            With<LocalEntity>,
            Without<HasClientLoaded>,
            // the vanilla client waits for the chunk mesh to be "compiled" for the renderer (or
            // some other conditions) before sending PlayerLoaded. see LevelLoadStatusManager.tick
            // in the decompiled source
            With<InLoadedChunk>,
        ),
    >,
) {
    for entity in query.iter() {
        commands.trigger(SendGamePacketEvent::new(entity, ServerboundPlayerLoaded));
        commands.entity(entity).insert(HasClientLoaded);
    }
}
