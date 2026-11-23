use std::sync::Arc;

use azalea_client::{
    packet::{config::SendConfigPacketEvent, game::SendGamePacketEvent},
    test_utils::prelude::*,
};
use azalea_codec::Codec;
use azalea_core::registry_holder::dimension_type::DimensionKindElement;
use azalea_protocol::packets::{
    ConnectionProtocol,
    config::{
        self, ClientboundFinishConfiguration, ClientboundRegistryData, ServerboundConfigPacket,
    },
    game::{self, ServerboundGamePacket},
};
use azalea_registry::identifier::Identifier;
use bevy_ecs::observer::On;
use parking_lot::Mutex;

#[test]
fn reply_to_ping_with_pong() {
    let _lock = init();

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
        registry_id: Identifier::new("minecraft:dimension_type"),
        entries: vec![(
            Identifier::new("minecraft:overworld"),
            Some(
                DimensionKindElement {
                    min_y: -64,
                    height: 384,
                    ..Default::default()
                }
                .codec_write(Default::default())
                .unwrap(),
            ),
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
