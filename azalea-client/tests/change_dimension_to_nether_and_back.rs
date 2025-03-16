use azalea_client::{InConfigState, InGameState, test_simulation::*};
use azalea_core::{position::ChunkPos, resource_location::ResourceLocation};
use azalea_entity::LocalEntity;
use azalea_protocol::packets::{
    ConnectionProtocol, Packet,
    config::{ClientboundFinishConfiguration, ClientboundRegistryData},
};
use azalea_registry::DimensionType;
use azalea_world::InstanceName;
use bevy_log::tracing_subscriber;
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn test_change_dimension_to_nether_and_back() {
    generic_test_change_dimension_to_nether_and_back(true);
    generic_test_change_dimension_to_nether_and_back(false);
}

fn generic_test_change_dimension_to_nether_and_back(using_respawn: bool) {
    let make_basic_login_or_respawn_packet = if using_respawn {
        |dimension: DimensionType, instance_name: ResourceLocation| {
            make_basic_respawn_packet(dimension, instance_name).into_variant()
        }
    } else {
        |dimension: DimensionType, instance_name: ResourceLocation| {
            make_basic_login_packet(dimension, instance_name).into_variant()
        }
    };

    let _ = tracing_subscriber::fmt::try_init();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);
    assert!(simulation.has_component::<InConfigState>());
    assert!(!simulation.has_component::<InGameState>());

    simulation.receive_packet(ClientboundRegistryData {
        registry_id: ResourceLocation::new("minecraft:dimension_type"),
        entries: vec![
            (
                // this dimension should never be created. it just exists to make sure we're not
                // hard-coding the dimension type id anywhere.
                ResourceLocation::new("azalea:fakedimension"),
                Some(NbtCompound::from_values(vec![
                    ("height".into(), NbtTag::Int(16)),
                    ("min_y".into(), NbtTag::Int(0)),
                ])),
            ),
            (
                ResourceLocation::new("minecraft:overworld"),
                Some(NbtCompound::from_values(vec![
                    ("height".into(), NbtTag::Int(384)),
                    ("min_y".into(), NbtTag::Int(-64)),
                ])),
            ),
            (
                ResourceLocation::new("minecraft:nether"),
                Some(NbtCompound::from_values(vec![
                    ("height".into(), NbtTag::Int(256)),
                    ("min_y".into(), NbtTag::Int(0)),
                ])),
            ),
        ]
        .into_iter()
        .collect(),
    });
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
        DimensionType::new_raw(1), // overworld
        ResourceLocation::new("azalea:a"),
    ));
    simulation.tick();

    assert_eq!(
        *simulation.component::<InstanceName>(),
        ResourceLocation::new("azalea:a"),
        "InstanceName should be azalea:a after setting dimension to that"
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
        DimensionType::new_raw(2), // nether
        ResourceLocation::new("azalea:b"),
    ));
    simulation.tick();

    assert!(
        simulation.chunk(ChunkPos::new(0, 0)).is_none(),
        "chunk should not exist immediately after changing dimensions"
    );
    assert_eq!(
        *simulation.component::<InstanceName>(),
        ResourceLocation::new("azalea:b"),
        "InstanceName should be azalea:b after changing dimensions to that"
    );

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), 256 / 16));
    simulation.tick();
    // make sure the chunk exists
    simulation
        .chunk(ChunkPos::new(0, 0))
        .expect("chunk should exist");
    simulation.receive_packet(make_basic_login_or_respawn_packet(
        DimensionType::new_raw(2), // nether
        ResourceLocation::new("minecraft:nether"),
    ));
    simulation.tick();

    //
    // BACK TO OVERWORLD
    //

    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(1), // overworld
        ResourceLocation::new("azalea:a"),
    ));
    simulation.tick();

    assert_eq!(
        *simulation.component::<InstanceName>(),
        ResourceLocation::new("azalea:a"),
        "InstanceName should be azalea:a after setting dimension back to that"
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
