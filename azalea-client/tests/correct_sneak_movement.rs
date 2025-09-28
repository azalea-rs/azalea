use azalea_client::{PhysicsState, StartWalkEvent, WalkDirection, test_utils::prelude::*};
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
fn test_correct_sneak_movement() {
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

    simulation.with_component_mut::<PhysicsState>(|p| p.trying_to_crouch = true);
    simulation.tick();
    sent_packets.expect("PlayerInput", |p| {
        matches!(
            p,
            ServerboundGamePacket::PlayerInput(p)
            if *p == ServerboundPlayerInput { shift: true, ..Default::default() }
        )
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    simulation.tick();
    simulation.write_message(StartWalkEvent {
        entity: simulation.entity,
        direction: WalkDirection::Forward,
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    simulation.tick();
    sent_packets.expect("PlayerInput", |p| {
        matches!(
            p,
            ServerboundGamePacket::PlayerInput(p)
            if *p == ServerboundPlayerInput { forward: true, shift: true, ..Default::default() }
        )
    });
    sent_packets.expect("MovePlayerPos { z: 0.5294000033944846 }", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPos(p)
            if p.pos == Vec3::new(0.5, 120., 0.5294000033944846)
        )
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    simulation.tick();
    sent_packets.expect("MovePlayerPos { z: 0.5748524105068866 }", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPos(p)
            if p.pos == Vec3::new(0.5, 120., 0.5748524105068866)
        )
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    simulation.tick();
    sent_packets.expect("MovePlayerPos: { z: 0.6290694310673044 }", |p| {
        matches!(
            p,
            ServerboundGamePacket::MovePlayerPos(p)
            if p.pos == Vec3::new(0.5, 120., 0.6290694310673044)
        )
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();
}
