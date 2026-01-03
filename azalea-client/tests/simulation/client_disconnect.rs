use azalea_client::test_utils::prelude::*;
use azalea_protocol::packets::ConnectionProtocol;
use azalea_world::InstanceName;

#[test]
fn test_client_disconnect() {
    let _lock = init();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);

    simulation.disconnect();
    simulation.tick();

    // make sure we're disconnected
    let is_connected = simulation.has_component::<InstanceName>();
    assert!(!is_connected);

    // tick again to make sure nothing goes wrong
    simulation.tick();
}
