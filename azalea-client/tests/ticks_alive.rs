use azalea_client::{test_utils::prelude::*, tick_counter::TicksConnected};
use azalea_protocol::packets::ConnectionProtocol;

#[test]
fn counter_increments_and_resets_on_disconnect() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    simulation.tick();

    assert!(!simulation.has_component::<TicksConnected>());
    simulation.receive_packet(default_login_packet());
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
