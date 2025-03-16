use azalea_client::test_simulation::*;
use azalea_core::{position::ChunkPos, resource_location::ResourceLocation};
use azalea_entity::metadata::Cow;
use azalea_protocol::packets::{ConnectionProtocol, game::ClientboundMoveEntityRot};
use azalea_registry::{DimensionType, EntityKind};
use azalea_world::MinecraftEntityId;
use bevy_ecs::query::With;
use bevy_log::tracing_subscriber;

#[test]
fn test_move_despawned_entity() {
    let _ = tracing_subscriber::fmt::try_init();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(0),
        ResourceLocation::new("azalea:overworld"),
    ));

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.tick();
    // spawn a cow
    simulation.receive_packet(make_basic_add_entity(EntityKind::Cow, 123, (0.5, 64., 0.5)));
    simulation.tick();

    // make sure it's spawned
    let mut cow_query = simulation.app.world_mut().query_filtered::<(), With<Cow>>();
    let cow_iter = cow_query.iter(simulation.app.world());
    assert_eq!(cow_iter.count(), 1, "cow should be despawned");

    // despawn the cow by receiving a login packet
    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(0),
        ResourceLocation::new("azalea:overworld"),
    ));
    simulation.tick();

    // make sure it's despawned
    let mut cow_query = simulation.app.world_mut().query_filtered::<(), With<Cow>>();
    let cow_iter = cow_query.iter(simulation.app.world());
    assert_eq!(cow_iter.count(), 0, "cow should be despawned");

    // send a move_entity_rot
    simulation.receive_packet(ClientboundMoveEntityRot {
        entity_id: MinecraftEntityId(123),
        y_rot: 0,
        x_rot: 0,
        on_ground: false,
    });
    simulation.tick();
}
