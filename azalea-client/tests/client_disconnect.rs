use azalea_client::test_simulation::*;
use azalea_protocol::packets::ConnectionProtocol;
use azalea_world::InstanceName;
use bevy_log::tracing_subscriber;

#[test]
fn test_client_disconnect() {
    let _ = tracing_subscriber::fmt::try_init();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);

    simulation.disconnect();
    simulation.tick();

    // make sure we're disconnected
    let is_connected = simulation.has_component::<InstanceName>();
    assert!(!is_connected);

    // tick again to make sure nothing goes wrong
    simulation.tick();
}
