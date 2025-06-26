use std::{collections::VecDeque, sync::Arc};

use azalea_client::{
    SprintDirection, StartSprintEvent, packet::game::SendPacketEvent, test_utils::prelude::*,
};
use azalea_core::{
    position::{BlockPos, ChunkPos, Vec3},
    resource_location::ResourceLocation,
};
use azalea_entity::LookDirection;
use azalea_protocol::{
    common::movements::{MoveFlags, PositionMoveRotation, RelativeMovements},
    packets::{
        ConnectionProtocol,
        config::{ClientboundFinishConfiguration, ClientboundRegistryData},
        game::{
            ClientboundBlockUpdate, ClientboundPlayerPosition, ServerboundAcceptTeleportation,
            ServerboundGamePacket, ServerboundMovePlayerPos, ServerboundMovePlayerPosRot,
            ServerboundMovePlayerStatusOnly,
        },
    },
};
use azalea_registry::{Block, DataRegistry, DimensionType};
use bevy_ecs::observer::Trigger;
use parking_lot::Mutex;
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn test_packet_order() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);
    let sent_packets = SentPackets::new(&mut simulation);

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
    simulation.tick();
    simulation.receive_packet(ClientboundFinishConfiguration);
    simulation.tick();

    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(0), // overworld
        ResourceLocation::new("minecraft:overworld"),
    ));
    simulation.tick();

    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    // receive a chunk so the player is "loaded" now
    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.receive_packet(ClientboundBlockUpdate {
        pos: BlockPos::new(1, 1, 3),
        block_state: Block::Stone.into(),
    });
    simulation.receive_packet(ClientboundPlayerPosition {
        id: 1,
        change: PositionMoveRotation {
            pos: Vec3::new(1.5, 2., 3.5),
            delta: Vec3::ZERO,
            look_direction: LookDirection::default(),
        },
        relative: RelativeMovements::all_absolute(),
    });
    simulation.tick();

    assert_eq!(
        simulation.get_block_state(BlockPos::new(1, 1, 3)),
        Some(Block::Stone.into())
    );

    println!("sent_packets: {:?}", sent_packets.list.lock());
    sent_packets.expect("AcceptTeleportation", |p| {
        matches!(
            p,
            ServerboundGamePacket::AcceptTeleportation(ServerboundAcceptTeleportation { id: 1 })
        )
    });
    sent_packets.expect("MovePlayerPosRot", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPosRot(ServerboundMovePlayerPosRot {
                flags: MoveFlags {
                    on_ground: false,
                    horizontal_collision: false
                },
                ..
            })
        )
    });

    // in vanilla these might be sent in a later tick (depending on how long it
    // takes to render the chunks)... see the comment in player_loaded_packet.
    // this might be worth changing later for better anticheat compat?
    sent_packets.expect("PlayerLoaded", |p| {
        matches!(p, ServerboundGamePacket::PlayerLoaded(_))
    });
    sent_packets.expect("MovePlayerPos", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPos(ServerboundMovePlayerPos {
                flags: MoveFlags {
                    on_ground: false,
                    horizontal_collision: false
                },
                ..
            })
        )
    });

    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    // it takes a tick for on_ground to be true
    simulation.tick();
    sent_packets.expect("MovePlayerStatusOnly", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerStatusOnly(ServerboundMovePlayerStatusOnly {
                flags: MoveFlags {
                    on_ground: true,
                    horizontal_collision: false
                }
            })
        )
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    // make sure nothing happens now
    simulation.tick();
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    // now sprint for a tick
    simulation.send_event(StartSprintEvent {
        entity: simulation.entity,
        direction: SprintDirection::Forward,
    });
    simulation.tick();
    sent_packets.expect("PlayerInput", |p| {
        matches!(p, ServerboundGamePacket::PlayerInput(_))
    });
    sent_packets.expect("PlayerCommand", |p| {
        matches!(p, ServerboundGamePacket::PlayerCommand(_))
    });
    sent_packets.expect("MovePlayerPos", |p| {
        matches!(p, ServerboundGamePacket::MovePlayerPos(_))
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();
}

#[derive(Clone)]
pub struct SentPackets {
    list: Arc<Mutex<VecDeque<ServerboundGamePacket>>>,
}
impl SentPackets {
    pub fn new(simulation: &mut Simulation) -> Self {
        let sent_packets = SentPackets {
            list: Default::default(),
        };

        let simulation_entity = simulation.entity;
        let sent_packets_clone = sent_packets.clone();
        simulation
            .app
            .add_observer(move |trigger: Trigger<SendPacketEvent>| {
                if trigger.sent_by == simulation_entity {
                    sent_packets_clone
                        .list
                        .lock()
                        .push_back(trigger.event().packet.clone())
                }
            });

        sent_packets
    }

    pub fn clear(&self) {
        self.list.lock().clear();
    }

    pub fn expect_tick_end(&self) {
        self.expect("TickEnd", |p| {
            matches!(p, ServerboundGamePacket::ClientTickEnd(_))
        });
    }
    pub fn expect_empty(&self) {
        let sent_packet = self.next();
        if let None = sent_packet {
        } else {
            panic!("Expected no packet, got {sent_packet:?}");
        }
    }
    pub fn expect(
        &self,
        expected_formatted: &str,
        check: impl FnOnce(&ServerboundGamePacket) -> bool,
    ) {
        let sent_packet = self.next();
        if let Some(sent_packet) = sent_packet {
            if !check(&sent_packet) {
                panic!("Expected {expected_formatted}, got {sent_packet:?}");
            }
        } else {
            panic!("Expected {expected_formatted}, got nothing");
        }
    }
    pub fn next(&self) -> Option<ServerboundGamePacket> {
        self.list.lock().pop_front()
    }
}
