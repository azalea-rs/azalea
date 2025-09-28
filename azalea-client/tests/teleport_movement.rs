use azalea_client::test_utils::prelude::*;
use azalea_core::{
    delta::PositionDelta8,
    position::{BlockPos, ChunkPos, Vec3},
};
use azalea_entity::LookDirection;
use azalea_protocol::{
    common::movements::{MoveFlags, PositionMoveRotation, RelativeMovements},
    packets::{
        ConnectionProtocol,
        game::{
            ClientboundBlockUpdate, ClientboundForgetLevelChunk, ClientboundPing,
            ClientboundPlayerPosition, ClientboundSetChunkCacheCenter, ClientboundSetEntityMotion,
            ServerboundGamePacket, ServerboundMovePlayerPos, ServerboundMovePlayerPosRot,
        },
    },
};
use azalea_registry::Block;
use azalea_world::MinecraftEntityId;

#[test]
fn test_teleport_movement() {
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
    sent_packets.clear();

    // now teleport to a far-away location
    tracing::info!("meow!");
    simulation.receive_packet(ClientboundPing { id: 1 });
    simulation.receive_packet(ClientboundPlayerPosition {
        id: 2,
        change: PositionMoveRotation {
            pos: Vec3::new(10000.5, 70.0, 0.5),
            delta: Vec3::ZERO,
            look_direction: LookDirection::default(),
        },
        relative: RelativeMovements::all_absolute(),
    });
    simulation.receive_packet(ClientboundPing { id: 2 });
    simulation.tick();
    sent_packets.expect_pong(1);
    sent_packets.expect("AcceptTeleportation", |p| {
        matches!(p, ServerboundGamePacket::AcceptTeleportation(_))
    });
    sent_packets.expect("MovePlayerPosRot", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPosRot(p)
            if p == &ServerboundMovePlayerPosRot {
                pos: Vec3::new(10000.5, 70.0, 0.5),
                flags: MoveFlags::default(),
                look_direction: LookDirection::default(),
            }
        )
    });
    sent_packets.expect_pong(2);
    sent_packets.expect("MovePlayerPos", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPos(p)
            if p == &ServerboundMovePlayerPos {
                pos: Vec3::new(10000.5, 70.0, 0.5),
                flags: MoveFlags::default()
            }
        )
    });

    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    //

    simulation.receive_packet(ClientboundForgetLevelChunk {
        pos: ChunkPos { x: 1, z: 23 },
    });
    simulation.receive_packet(ClientboundSetChunkCacheCenter { x: 625, z: 0 });
    simulation.receive_packet(ClientboundPing { id: 3 });
    simulation.receive_packet(ClientboundPlayerPosition {
        id: 3,
        change: PositionMoveRotation {
            pos: Vec3::new(10000.5, 70.0000001, 0.5),
            delta: Vec3::ZERO,
            look_direction: LookDirection::default(),
        },
        relative: RelativeMovements::all_absolute(),
    });
    simulation.receive_packet(ClientboundPing { id: 4 });
    simulation.receive_packet(ClientboundSetEntityMotion {
        id: MinecraftEntityId(0),
        delta: PositionDelta8 {
            xa: 0,
            ya: -627,
            za: 0,
        },
    });
    simulation.receive_packet(ClientboundPing { id: 5 });
    simulation.tick();

    sent_packets.expect_pong(3);
    sent_packets.expect("AcceptTeleportation", |p| {
        matches!(p, ServerboundGamePacket::AcceptTeleportation(_))
    });
    sent_packets.expect("MovePlayerPosRot", |p| {
        matches!(
        p,
        ServerboundGamePacket::MovePlayerPosRot(p)
        if p == &ServerboundMovePlayerPosRot {
            pos: Vec3::new(10000.5, 70.0000001, 0.5),
            flags: MoveFlags::default(),
            look_direction: LookDirection::default(),
        })
    });
    sent_packets.expect_pong(4);
    sent_packets.expect_pong(5);
    sent_packets.expect("MovePlayerPos", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPos(p)
            if p == &ServerboundMovePlayerPos {
                pos: Vec3::new(10000.5, 69.9216251, 0.5),
                flags: MoveFlags::default()
            }
        )
    });

    sent_packets.expect_tick_end();
    sent_packets.expect_empty();
}
