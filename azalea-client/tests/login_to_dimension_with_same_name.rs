use azalea_client::{InConfigState, InGameState, local_player::InstanceHolder, test_simulation::*};
use azalea_core::{position::ChunkPos, resource_location::ResourceLocation};
use azalea_entity::LocalEntity;
use azalea_protocol::packets::{
    ConnectionProtocol, Packet,
    config::{ClientboundFinishConfiguration, ClientboundRegistryData},
    game::ClientboundStartConfiguration,
};
use azalea_registry::{DataRegistry, DimensionType};
use azalea_world::InstanceName;
use bevy_log::tracing_subscriber;
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn test_login_to_dimension_with_same_name() {
    let _ = tracing_subscriber::fmt().try_init();

    generic_test_login_to_dimension_with_same_name(true);
    generic_test_login_to_dimension_with_same_name(false);
}

fn generic_test_login_to_dimension_with_same_name(using_respawn: bool) {
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
        entries: vec![(
            ResourceLocation::new("minecraft:overworld"),
            Some(NbtCompound::from_values(vec![
                ("height".into(), NbtTag::Int(384)),
                ("min_y".into(), NbtTag::Int(-64)),
            ])),
        )]
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
    // OVERWORLD 1
    //

    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(0), // overworld
        ResourceLocation::new("azalea:overworld"),
    ));
    simulation.tick();

    assert_eq!(
        *simulation.component::<InstanceName>(),
        ResourceLocation::new("azalea:overworld"),
        "InstanceName should be azalea:overworld after setting dimension to that"
    );

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.tick();
    // make sure the chunk exists
    simulation
        .chunk(ChunkPos::new(0, 0))
        .expect("chunk should exist");

    //
    // OVERWORLD 2
    //

    simulation.receive_packet(ClientboundStartConfiguration);
    simulation.receive_packet(ClientboundRegistryData {
        registry_id: ResourceLocation::new("minecraft:dimension_type"),
        entries: vec![(
            ResourceLocation::new("minecraft:overworld"),
            Some(NbtCompound::from_values(vec![
                ("height".into(), NbtTag::Int(256)),
                ("min_y".into(), NbtTag::Int(0)),
            ])),
        )]
        .into_iter()
        .collect(),
    });
    simulation.receive_packet(ClientboundFinishConfiguration);
    simulation.receive_packet(make_basic_login_or_respawn_packet(
        DimensionType::new_raw(0),
        ResourceLocation::new("azalea:overworld"),
    ));
    simulation.tick();

    assert!(
        simulation.chunk(ChunkPos::new(0, 0)).is_none(),
        "chunk should not exist immediately after changing dimensions"
    );
    assert_eq!(
        *simulation.component::<InstanceName>(),
        ResourceLocation::new("azalea:overworld"),
        "InstanceName should still be azalea:overworld after changing dimensions to that"
    );
    assert_eq!(
        simulation
            .component::<InstanceHolder>()
            .instance
            .read()
            .chunks
            .height,
        256
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
}
