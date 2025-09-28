use azalea_client::{PhysicsState, SprintDirection, StartSprintEvent, test_utils::prelude::*};
use azalea_core::position::{BlockPos, ChunkPos, Vec3};
use azalea_entity::LookDirection;
use azalea_protocol::{
    common::movements::{PositionMoveRotation, RelativeMovements},
    packets::{
        ConnectionProtocol,
        game::{
            ClientboundBlockUpdate, ClientboundPlayerPosition, ServerboundGamePacket,
            ServerboundPlayerInput,
        },
    },
};
use azalea_registry::Block;

#[test]
fn test_correct_sprint_sneak_movement() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    let sent_packets = SentPackets::new(&mut simulation);

    simulation.receive_packet(default_login_packet());
    simulation.tick();

    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.receive_packet(ClientboundBlockUpdate {
        pos: BlockPos::new(0, 119, 0),
        block_state: Block::Stone.into(),
    });
    simulation.receive_packet(ClientboundBlockUpdate {
        pos: BlockPos::new(0, 119, 1),
        block_state: Block::Stone.into(),
    });
    simulation.receive_packet(ClientboundPlayerPosition {
        id: 1,
        change: PositionMoveRotation {
            pos: Vec3::new(0.5, 120., 0.5),
            delta: Vec3::ZERO,
            look_direction: LookDirection::default(),
        },
        relative: RelativeMovements::all_absolute(),
    });
    simulation.tick();
    simulation.tick();
    simulation.tick();
    sent_packets.clear();

    // start sprinting
    simulation.write_message(StartSprintEvent {
        entity: simulation.entity,
        direction: SprintDirection::Forward,
    });
    simulation.tick();
    sent_packets.expect("PlayerInput", |p| {
        matches!(
            p,
            ServerboundGamePacket::PlayerInput(p)
            if *p == ServerboundPlayerInput { forward: true, sprint: true, ..Default::default() }
        )
    });
    sent_packets.expect("PlayerCommand", |p| {
        matches!(p, ServerboundGamePacket::PlayerCommand(_))
    });
    sent_packets.expect("MovePlayerPos { z: 0.6274000124096872 }", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPos(p)
            if p.pos == Vec3::new(0.5, 120., 0.6274000124096872)
        )
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    simulation.tick();
    sent_packets.expect("MovePlayerPos { z: 0.8243604396746886 }", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPos(p)
            if p.pos == Vec3::new(0.5, 120., 0.8243604396746886)
        )
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();
    simulation.with_component_mut::<PhysicsState>(|p| p.trying_to_crouch = true);

    simulation.tick();
    sent_packets.expect("PlayerInput", |p| {
        matches!(
            p,
            ServerboundGamePacket::PlayerInput(p)
            if *p == ServerboundPlayerInput { forward: true, sprint: true, shift: true, ..Default::default() }
        )
    });
    sent_packets.expect("MovePlayerPos { z: 1.0593008578621674 }", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPos(p)
            if p.pos == Vec3::new(0.5, 120., 1.0593008578621674)
        )
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    simulation.tick();
    sent_packets.expect("MovePlayerPos { z: 1.2257983479146455 }", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPos(p)
            if p.pos == Vec3::new(0.5, 120., 1.2257983479146455)
        )
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    simulation.tick();
    sent_packets.expect("MovePlayerPos: { z: 1.3549259948648078 }", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPos(p)
            if p.pos == Vec3::new(0.5, 120., 1.3549259948648078)
        )
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();
}
