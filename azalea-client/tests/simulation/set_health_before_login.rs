use azalea_client::{InConfigState, test_utils::prelude::*};
use azalea_entity::{LocalEntity, metadata::Health};
use azalea_protocol::packets::{
    ConnectionProtocol, config::ClientboundFinishConfiguration, game::ClientboundSetHealth,
};

#[test]
fn test_set_health_before_login() {
    let _lock = init();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);
    assert!(simulation.has_component::<InConfigState>());

    receive_default_registries(&mut simulation);
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
