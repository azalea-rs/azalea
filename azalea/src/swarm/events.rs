use azalea_client::LocalPlayer;
use azalea_ecs::{
    app::{App, Plugin},
    event::EventWriter,
    query::With,
    system::{Query, ResMut, Resource},
};
use azalea_world::entity::MinecraftEntityId;
use derive_more::{Deref, DerefMut};

pub struct SwarmPlugin;
impl Plugin for SwarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SwarmReadyEvent>()
            .add_system(check_ready)
            .init_resource::<IsSwarmReady>();
    }
}

/// All the bots from the swarm are now in the world.
pub struct SwarmReadyEvent;

#[derive(Default, Resource, Deref, DerefMut)]
struct IsSwarmReady(bool);

fn check_ready(
    query: Query<Option<&MinecraftEntityId>, With<LocalPlayer>>,
    mut is_swarm_ready: ResMut<IsSwarmReady>,
    mut ready_events: EventWriter<SwarmReadyEvent>,
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
    ready_events.send(SwarmReadyEvent);
}
