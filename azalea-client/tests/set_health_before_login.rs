use azalea_client::{InConfigState, test_utils::prelude::*};
use azalea_core::resource_location::ResourceLocation;
use azalea_entity::{LocalEntity, metadata::Health};
use azalea_protocol::packets::{
    ConnectionProtocol,
    config::{ClientboundFinishConfiguration, ClientboundRegistryData},
    game::ClientboundSetHealth,
};
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn test_set_health_before_login() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);
    assert!(simulation.has_component::<InConfigState>());

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
    assert!(simulation.has_component::<LocalEntity>());

    simulation.receive_packet(ClientboundSetHealth {
        health: 15.,
        food: 20,
        saturation: 20.,
    });
    simulation.tick();
    assert_eq!(*simulation.component::<Health>(), 15.);

    simulation.receive_packet(default_login_packet());
    simulation.tick();

    // health should stay the same
    assert_eq!(*simulation.component::<Health>(), 15.);
}
