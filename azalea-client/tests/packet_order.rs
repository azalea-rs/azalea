use azalea_client::{SprintDirection, StartSprintEvent, test_utils::prelude::*};
use azalea_core::position::{BlockPos, ChunkPos, Vec3};
use azalea_entity::LookDirection;
use azalea_protocol::{
    common::movements::{MoveFlags, PositionMoveRotation, RelativeMovements},
    packets::{
        ConnectionProtocol,
        game::{
            ClientboundBlockUpdate, ClientboundPlayerPosition, ServerboundAcceptTeleportation,
            ServerboundGamePacket, ServerboundMovePlayerPosRot, ServerboundMovePlayerStatusOnly,
        },
    },
};
use azalea_registry::Block;

#[test]
fn test_packet_order() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    let sent_packets = SentPackets::new(&mut simulation);

    simulation.receive_packet(default_login_packet());
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
    sent_packets.expect("AcceptTeleportation", |p| {
        matches!(
            p,
            ServerboundGamePacket::AcceptTeleportation(ServerboundAcceptTeleportation { id: 1 })
        )
    });
    sent_packets.expect("MovePlayerPosRot", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPosRot(p)
            if p == &ServerboundMovePlayerPosRot {
                flags: MoveFlags {
                    on_ground: false,
                    horizontal_collision: false
                },
                pos: Vec3::new(1.5, 2., 3.5),
                look_direction: LookDirection::default(),
            }
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
            ServerboundGamePacket::MovePlayerPos(p)
            if p.flags == MoveFlags {
                on_ground: false,
                horizontal_collision: false
            }
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
    simulation.write_message(StartSprintEvent {
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
