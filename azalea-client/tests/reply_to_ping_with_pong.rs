use std::sync::Arc;

use azalea_client::{packet::game::SendPacketEvent, test_simulation::*};
use azalea_protocol::packets::{
    ConnectionProtocol,
    game::{ClientboundPing, ServerboundGamePacket},
};
use bevy_ecs::observer::Trigger;
use bevy_log::tracing_subscriber;
use parking_lot::Mutex;

#[test]
fn reply_to_ping_with_pong() {
    let _ = tracing_subscriber::fmt::try_init();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    let reply_count = Arc::new(Mutex::new(0));
    let reply_count_clone = reply_count.clone();
    simulation
        .app
        .add_observer(move |trigger: Trigger<SendPacketEvent>| {
            if trigger.sent_by == simulation.entity {
                if let ServerboundGamePacket::Pong(packet) = &trigger.packet {
                    assert_eq!(packet.id, 123);
                    *reply_count_clone.lock() += 1;
                }
            }
        });

    simulation.tick();
    simulation.receive_packet(ClientboundPing { id: 123 });
    simulation.tick();

    assert_eq!(*reply_count.lock(), 1);
}
