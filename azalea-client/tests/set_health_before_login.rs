use azalea_client::{test_simulation::*, InConfigState};
use azalea_core::resource_location::ResourceLocation;
use azalea_entity::{metadata::Health, LocalEntity};
use azalea_protocol::packets::{
    config::{ClientboundFinishConfiguration, ClientboundRegistryData},
    game::ClientboundSetHealth,
    ConnectionProtocol,
};
use azalea_registry::DimensionType;
use bevy_log::tracing_subscriber;
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn test_set_health_before_login() {
    let _ = tracing_subscriber::fmt::try_init();

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

    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(0), // overworld
        ResourceLocation::new("minecraft:overworld"),
    ));
    simulation.tick();

    // health should stay the same
    assert_eq!(*simulation.component::<Health>(), 15.);
}
