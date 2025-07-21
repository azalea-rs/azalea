use azalea_client::{test_utils::prelude::*, tick_counter::TicksConnected};
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol::packets::{config::{ClientboundFinishConfiguration, ClientboundRegistryData}, ConnectionProtocol};
use azalea_registry::{DataRegistry, DimensionType};
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn counter_increments_and_resets_on_disconnect() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);

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

    simulation.receive_packet(ClientboundFinishConfiguration);

    simulation.tick();
    // we need a second tick to handle the state switch properly
    simulation.tick();

    assert!(!simulation.has_component::<TicksConnected>());

    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(0), // overworld
        ResourceLocation::new("minecraft:overworld"),
    ));
    simulation.tick();

    assert!(simulation.has_component::<TicksConnected>());
    assert_eq!(simulation.component::<TicksConnected>().0, 1);

    // Tick three times; counter should read 2, 3, 4.
    for expected in 2..=4 {
        simulation.tick();
        let counter = simulation.component::<TicksConnected>();
        assert_eq!(
            counter.0, expected,
            "after {expected} tick(s) counter should be {expected}"
        );
    }

    simulation.disconnect();
    simulation.tick();

    assert!(!simulation.has_component::<TicksConnected>());
}
