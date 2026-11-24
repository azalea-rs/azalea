use azalea_client::test_utils::prelude::*;
use azalea_core::{identifier::Identifier, position::ChunkPos};
use azalea_entity::metadata::Cow;
use azalea_protocol::packets::{ConnectionProtocol, config::ClientboundFinishConfiguration};
use azalea_registry::{builtin::EntityKind, data::DimensionKindKey};
use bevy_ecs::query::With;

#[test]
fn test_despawn_entities_when_changing_dimension() {
    let _lock = init();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);
    receive_default_registries(&mut simulation);
    simulation.tick();
    simulation.receive_packet(ClientboundFinishConfiguration);
    simulation.tick();

    //
    // OVERWORLD
    //

    simulation.receive_packet(make_basic_login_packet(
        DimensionKindKey::Overworld,
        Identifier::new("azalea:a"),
        &simulation,
    ));
    simulation.tick();

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.tick();
    // spawn a cow
    simulation.receive_packet(make_basic_add_entity(EntityKind::Cow, 123, (0.5, 64., 0.5)));
    simulation.tick();
    // make sure it's spawned
    let mut cow_query = simulation.app.world_mut().query_filtered::<(), With<Cow>>();
    let cow_iter = cow_query.iter(simulation.app.world());
    assert_eq!(cow_iter.count(), 1, "cow should be spawned");

    //
    // NETHER
    //

    simulation.receive_packet(make_basic_respawn_packet(
        DimensionKindKey::TheNether,
        Identifier::new("azalea:b"),
        &simulation,
    ));
    simulation.tick();

    // cow should be completely deleted from the ecs
    let cow_iter = cow_query.iter(simulation.app.world());
    assert_eq!(
        cow_iter.count(),
        0,
        "cow should be despawned after switching dimensions"
    );
}
