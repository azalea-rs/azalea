use azalea_client::{StartWalkEvent, WalkDirection, test_utils::prelude::*};
use azalea_core::position::{BlockPos, ChunkPos, Vec3};
use azalea_entity::LookDirection;
use azalea_protocol::{
    common::movements::{MoveFlags, PositionMoveRotation, RelativeMovements},
    packets::{
        ConnectionProtocol,
        game::{
            ClientboundBlockUpdate, ClientboundPlayerPosition, ClientboundSetChunkCacheCenter,
            ServerboundGamePacket, ServerboundMovePlayerPos,
        },
    },
};
use azalea_registry::Block;

#[test]
fn test_correct_movement() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    let sent_packets = SentPackets::new(&mut simulation);

    simulation.receive_packet(default_login_packet());
    simulation.tick();

    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    // receive a chunk so the player is "loaded" now
    simulation.receive_packet(ClientboundSetChunkCacheCenter { x: 1, z: 23 });
    simulation.receive_packet(make_basic_empty_chunk(
        ChunkPos::new(1, 23),
        (384 + 64) / 16,
    ));
    simulation.receive_packet(ClientboundBlockUpdate {
        pos: BlockPos::new(31, 63, 370),
        block_state: Block::Stone.into(),
    });
    simulation.receive_packet(ClientboundPlayerPosition {
        id: 1,
        change: PositionMoveRotation {
            pos: Vec3::new(31.5, 64., 370.5),
            delta: Vec3::ZERO,
            look_direction: LookDirection::default(),
        },
        relative: RelativeMovements::all_absolute(),
    });
    simulation.tick();
    simulation.tick();

    // walk for a tick
    simulation.write_message(StartWalkEvent {
        entity: simulation.entity,
        direction: WalkDirection::Forward,
    });
    sent_packets.clear();
    simulation.tick();
    sent_packets.expect("PlayerInput", |p| {
        matches!(p, ServerboundGamePacket::PlayerInput(_))
    });
    sent_packets.expect("MovePlayerPos { pos.z: 370.59800000336764, ... }", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPos(ServerboundMovePlayerPos {
                pos: Vec3 {
                    x: 31.5,
                    y: 64.0,
                    z: 370.59800000336764
                },
                flags: MoveFlags {
                    on_ground: true,
                    horizontal_collision: false
                }
            })
        )
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();
}
