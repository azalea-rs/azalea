use azalea_client::{
    InConfigState, InGameState, local_player::WorldHolder, test_utils::prelude::*,
};
use azalea_core::{identifier::Identifier, position::ChunkPos};
use azalea_entity::LocalEntity;
use azalea_protocol::packets::{
    ConnectionProtocol, config::ClientboundFinishConfiguration, game::ClientboundStartConfiguration,
};
use azalea_registry::{
    data::DimensionKindKey, registry_holder::dimension_type::DimensionKindElement,
};
use azalea_world::WorldName;

#[test]
fn test_login_to_dimension_with_same_name() {
    let _lock = init();

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
    // OVERWORLD 1
    //

    simulation.receive_packet(make_basic_login_packet(
        DimensionKindKey::Overworld,
        Identifier::new("azalea:overworld"),
        &simulation,
    ));
    simulation.tick();

    assert_eq!(
        *simulation.component::<WorldName>(),
        Identifier::new("azalea:overworld"),
        "WorldName should be azalea:overworld after setting dimension to that"
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
    receive_default_registries_with_dimensions(
        &mut simulation,
        vec![(
            "overworld",
            DimensionKindElement {
                min_y: 0,
                height: 256,
                ..Default::default()
            },
        )],
    );
    simulation.receive_packet(ClientboundFinishConfiguration);
    simulation.receive_packet(make_basic_login_packet(
        DimensionKindKey::Overworld,
        Identifier::new("azalea:overworld"),
        &simulation,
    ));
    simulation.tick();

    assert!(
        simulation.chunk(ChunkPos::new(0, 0)).is_none(),
        "chunk should not exist immediately after changing dimensions"
    );
    assert_eq!(
        *simulation.component::<WorldName>(),
        Identifier::new("azalea:overworld"),
        "WorldName should still be azalea:overworld after changing dimensions to that"
    );
    assert_eq!(
        simulation
            .component::<WorldHolder>()
            .shared
            .read()
            .chunks
            .height(),
        256
    );

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), 256 / 16));
    simulation.tick();
    // make sure the chunk exists
    simulation
        .chunk(ChunkPos::new(0, 0))
        .expect("chunk should exist");
}
