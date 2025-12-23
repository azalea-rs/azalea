use azalea_client::{InConfigState, test_utils::prelude::*};
use azalea_protocol::packets::{ConnectionProtocol, game::ClientboundStartConfiguration};
use azalea_world::InstanceName;

#[test]
fn test_receive_start_config_packet() {
    let _lock = init();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);

    simulation.receive_packet(default_login_packet());
    simulation.tick();
    assert!(simulation.has_component::<InstanceName>());
    simulation.tick();

    simulation.receive_packet(ClientboundStartConfiguration);

    simulation.tick();
    assert!(simulation.has_component::<InConfigState>());
}
