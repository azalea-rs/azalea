use azalea_client::{InConfigState, packet::game::SendPacketEvent, test_utils::prelude::*};
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol::packets::{ConnectionProtocol, game::ClientboundStartConfiguration};
use azalea_registry::{DataRegistry, DimensionType};
use azalea_world::InstanceName;
use bevy_ecs::event::Events;

#[test]
fn test_receive_start_config_packet() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(0),
        ResourceLocation::new("minecraft:overworld"),
    ));
    simulation.tick();
    assert!(simulation.has_component::<InstanceName>());
    simulation.tick();

    // we shouldn't be using the `SendPacketEvent` event directly, we should be
    // using the trigger instead
    simulation.with_resource_mut::<Events<SendPacketEvent>>(|send_packet_events| {
        assert_eq!(send_packet_events.len(), 0);
    });

    simulation.receive_packet(ClientboundStartConfiguration);

    simulation.tick();
    assert!(simulation.has_component::<InConfigState>());

    // check again just in case
    simulation.with_resource_mut::<Events<SendPacketEvent>>(|send_packet_events| {
        assert_eq!(send_packet_events.len(), 0);
    });
}
