use azalea_client::{InConfigState, test_utils::prelude::*};
use azalea_core::identifier::Identifier;
use azalea_entity::metadata::Health;
use azalea_protocol::packets::{
    ConnectionProtocol,
    config::{ClientboundFinishConfiguration, ClientboundRegistryData},
    game::ClientboundSetHealth,
};
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn test_fast_login() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);
    assert!(simulation.has_component::<InConfigState>());

    simulation.receive_packet(ClientboundRegistryData {
        registry_id: Identifier::new("minecraft:dimension_type"),
        entries: vec![(
            Identifier::new("minecraft:overworld"),
            Some(NbtCompound::from_values(vec![
                ("height".into(), NbtTag::Int(384)),
                ("min_y".into(), NbtTag::Int(-64)),
            ])),
        )]
        .into_iter()
        .collect(),
    });

    simulation.receive_packet(ClientboundFinishConfiguration);
    // note that there's no simulation tick here
    simulation.receive_packet(ClientboundSetHealth {
        health: 15.,
        food: 20,
        saturation: 20.,
    });
    simulation.tick();
    // we need a second tick to handle the state switch properly
    simulation.tick();
    assert_eq!(*simulation.component::<Health>(), 15.);
}
