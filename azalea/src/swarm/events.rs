use azalea_client::local_player::InstanceHolder;
use azalea_world::MinecraftEntityId;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};

pub struct SwarmPlugin;
impl Plugin for SwarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SwarmReadyEvent>()
            .add_systems(Update, check_ready)
            .init_resource::<IsSwarmReady>();
    }
}

/// All the bots from the swarm are now in the world.
#[derive(Message)]
pub struct SwarmReadyEvent;

#[derive(Default, Resource, Deref, DerefMut)]
struct IsSwarmReady(bool);

fn check_ready(
    query: Query<Option<&MinecraftEntityId>, With<InstanceHolder>>,
    mut is_swarm_ready: ResMut<IsSwarmReady>,
    mut ready_events: MessageWriter<SwarmReadyEvent>,
) {
    // if we already know the swarm is ready, do nothing
    if **is_swarm_ready {
        return;
    }
    // if all the players are in the world, we're ready
    for entity_id in query.iter() {
        if entity_id.is_none() {
            return;
        }
    }

    // all the players are in the world, so we're ready
    **is_swarm_ready = true;
    ready_events.write(SwarmReadyEvent);
}
