use std::sync::Arc;

use azalea_client::{
    packet::{config::SendConfigPacketEvent, game::SendGamePacketEvent},
    test_utils::prelude::*,
};
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol::packets::{
    ConnectionProtocol,
    config::{
        self, ClientboundFinishConfiguration, ClientboundRegistryData, ServerboundConfigPacket,
    },
    game::{self, ServerboundGamePacket},
};
use bevy_ecs::observer::On;
use parking_lot::Mutex;
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn reply_to_ping_with_pong() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);

    let reply_count = Arc::new(Mutex::new(0));
    let reply_count_clone = reply_count.clone();
    simulation
        .app
        .add_observer(move |send_config_packet: On<SendConfigPacketEvent>| {
            if send_config_packet.sent_by == simulation.entity
                && let ServerboundConfigPacket::Pong(packet) = &send_config_packet.packet
            {
                assert_eq!(packet.id, 321);
                *reply_count_clone.lock() += 1;
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
        .add_observer(move |send_game_packet: On<SendGamePacketEvent>| {
            if send_game_packet.sent_by == simulation.entity
                && let ServerboundGamePacket::Pong(packet) = &send_game_packet.packet
            {
                assert_eq!(packet.id, 123);
                *reply_count_clone.lock() += 1;
            }
        });

    simulation.tick();
    simulation.receive_packet(game::ClientboundPing { id: 123 });
    simulation.tick();

    assert_eq!(*reply_count.lock(), 1);
}
