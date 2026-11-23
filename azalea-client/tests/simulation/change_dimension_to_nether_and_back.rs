use azalea_client::{InConfigState, InGameState, test_utils::prelude::*};
use azalea_core::{identifier::Identifier, position::ChunkPos};
use azalea_entity::LocalEntity;
use azalea_protocol::packets::{
    ConnectionProtocol, Packet,
    config::{ClientboundFinishConfiguration, ClientboundRegistryData},
};
use azalea_registry::{DataRegistry, data::DimensionKind};
use azalea_world::WorldName;
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn test_change_dimension_to_nether_and_back() {
    let _lock = init();

    generic_test_change_dimension_to_nether_and_back(true);
    generic_test_change_dimension_to_nether_and_back(false);
}

fn generic_test_change_dimension_to_nether_and_back(using_respawn: bool) {
    let make_basic_login_or_respawn_packet = if using_respawn {
        |dimension: DimensionKind, world_name: Identifier| {
            make_basic_respawn_packet(dimension, world_name).into_variant()
        }
    } else {
        |dimension: DimensionKind, world_name: Identifier| {
            make_basic_login_packet(dimension, world_name).into_variant()
        }
    };

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);
    assert!(simulation.has_component::<InConfigState>());
    assert!(!simulation.has_component::<InGameState>());

    receive_default_registries(&mut simulation);
    simulation.tick();
    simulation.receive_packet(ClientboundFinishConfiguration);
    simulation.tick();

    assert!(!simulation.has_component::<InConfigState>());
    assert!(simulation.has_component::<InGameState>());
    assert!(simulation.has_component::<LocalEntity>());

    //
    // OVERWORLD
    //

    simulation.receive_packet(make_basic_login_packet(
        DimensionKind::new_raw(1), // overworld
        Identifier::new("azalea:a"),
    ));
    simulation.tick();

    assert_eq!(
        *simulation.component::<WorldName>(),
        Identifier::new("azalea:a"),
        "WorldName should be azalea:a after setting dimension to that"
    );

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.tick();
    // make sure the chunk exists
    simulation
        .chunk(ChunkPos::new(0, 0))
        .expect("chunk should exist");

    //
    // NETHER
    //

    simulation.receive_packet(make_basic_login_or_respawn_packet(
        DimensionKind::new_raw(2), // nether
        Identifier::new("azalea:b"),
    ));
    simulation.tick();

    assert!(
        simulation.chunk(ChunkPos::new(0, 0)).is_none(),
        "chunk should not exist immediately after changing dimensions"
    );
    assert_eq!(
        *simulation.component::<WorldName>(),
        Identifier::new("azalea:b"),
        "WorldName should be azalea:b after changing dimensions to that"
    );

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), 256 / 16));
    simulation.tick();
    // make sure the chunk exists
    simulation
        .chunk(ChunkPos::new(0, 0))
        .expect("chunk should exist");
    simulation.receive_packet(make_basic_login_or_respawn_packet(
        DimensionKind::new_raw(2), // nether
        Identifier::new("minecraft:nether"),
    ));
    simulation.tick();

    //
    // BACK TO OVERWORLD
    //

    simulation.receive_packet(make_basic_login_packet(
        DimensionKind::new_raw(1), // overworld
        Identifier::new("azalea:a"),
    ));
    simulation.tick();

    assert_eq!(
        *simulation.component::<WorldName>(),
        Identifier::new("azalea:a"),
        "WorldName should be azalea:a after setting dimension back to that"
    );
    assert!(
        simulation.chunk(ChunkPos::new(0, 0)).is_none(),
        "chunk should not exist immediately after switching back to overworld"
    );

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.tick();
    // make sure the chunk exists
    simulation
        .chunk(ChunkPos::new(0, 0))
        .expect("chunk should exist");
}
