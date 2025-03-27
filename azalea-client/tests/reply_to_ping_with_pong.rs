use std::sync::Arc;

use azalea_client::{
    packet::{config::SendConfigPacketEvent, game::SendPacketEvent},
    test_simulation::*,
};
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol::packets::{
    ConnectionProtocol,
    config::{
        self, ClientboundFinishConfiguration, ClientboundRegistryData, ServerboundConfigPacket,
    },
    game::{self, ServerboundGamePacket},
};
use bevy_ecs::observer::Trigger;
use bevy_log::tracing_subscriber;
use parking_lot::Mutex;
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn reply_to_ping_with_pong() {
    let _ = tracing_subscriber::fmt::try_init();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);

    let reply_count = Arc::new(Mutex::new(0));
    let reply_count_clone = reply_count.clone();
    simulation
        .app
        .add_observer(move |trigger: Trigger<SendConfigPacketEvent>| {
            if trigger.sent_by == simulation.entity {
                if let ServerboundConfigPacket::Pong(packet) = &trigger.packet {
                    assert_eq!(packet.id, 321);
                    *reply_count_clone.lock() += 1;
                }
            }
        });

    simulation.receive_packet(config::ClientboundPing { id: 321 });
    simulation.tick();
    assert_eq!(*reply_count.lock(), 1);

    // move into game state and test ClientboundPing there

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
    simulation.receive_packet(game::ClientboundPing { id: 123 });
    simulation.tick();

    assert_eq!(*reply_count.lock(), 1);
}
