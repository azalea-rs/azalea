use azalea_client::{mining::StartMiningBlockEvent, test_utils::prelude::*};
use azalea_core::{
    direction::Direction,
    position::{BlockPos, ChunkPos, Vec3},
};
use azalea_entity::LookDirection;
use azalea_protocol::{
    common::movements::{PositionMoveRotation, RelativeMovements},
    packets::{
        ConnectionProtocol, Packet,
        game::{
            ClientboundBlockUpdate, ClientboundPlayerPosition, ServerboundGamePacket,
            ServerboundPlayerAction, ServerboundSwing, s_interact::InteractionHand,
            s_player_action,
        },
    },
};
use azalea_registry::builtin::BlockKind;

#[test]
fn test_mine_block_timing_hand() {
    let _lock = init();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    let sent_packets = SentPackets::new(&mut simulation);
    simulation.receive_packet(default_login_packet());

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.tick();

    let pos = BlockPos::new(0, 2, 0);
    let pos2 = BlockPos::new(0, 1, 0);
    simulation.receive_packet(ClientboundBlockUpdate {
        pos,
        block_state: BlockKind::Stone.into(),
    });
    simulation.receive_packet(ClientboundBlockUpdate {
        pos: pos2,
        block_state: BlockKind::Stone.into(),
    });
    simulation.receive_packet(ClientboundPlayerPosition {
        id: 1,
        change: PositionMoveRotation {
            pos: pos.up(1).center_bottom(),
            delta: Vec3::ZERO,
            look_direction: LookDirection::default(),
        },
        relative: RelativeMovements::all_absolute(),
    });
    simulation.tick();
    assert_eq!(
        simulation.get_block_state(pos),
        Some(BlockKind::Stone.into())
    );
    println!("set serverside stone");
    simulation.with_component_mut::<LookDirection>(|look| {
        // look down
        look.update_x_rot(90.);
    });

    simulation.tick();
    simulation.tick();
    simulation.tick();

    simulation.write_message(StartMiningBlockEvent {
        entity: simulation.entity,
        position: pos,
        force: false,
    });
    sent_packets.clear();
    simulation.tick();
    sent_packets.expect("PlayerAction", |p| {
        p == &ServerboundPlayerAction {
            action: s_player_action::Action::StartDestroyBlock,
            pos,
            direction: Direction::Up,
            seq: 1,
        }
        .into_variant()
    });
    sent_packets.expect("Swing 1", |p| {
        p == &ServerboundSwing {
            hand: InteractionHand::MainHand,
        }
        .into_variant()
    });
    sent_packets.expect("Swing 2", |p| {
        p == &ServerboundSwing {
            hand: InteractionHand::MainHand,
        }
        .into_variant()
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    for i in 3..=151 {
        simulation.tick();
        sent_packets.expect(&format!("Swing {i}"), |p| {
            p == &ServerboundSwing {
                hand: InteractionHand::MainHand,
            }
            .into_variant()
        });
        sent_packets.maybe_expect(|p| matches!(p, ServerboundGamePacket::MovePlayerPos(_)));
        sent_packets.expect_tick_end();
        sent_packets.expect_empty();
    }

    simulation.tick();
    sent_packets.expect(
        "ServerboundPlayerAction { action: StopDestroyBlock }",
        |p| {
            matches!(
                p,
                ServerboundGamePacket::PlayerAction(p)
                if p.action == s_player_action::Action::StopDestroyBlock
            )
        },
    );
    sent_packets.expect("Last swing", |p| {
        p == &ServerboundSwing {
            hand: InteractionHand::MainHand,
        }
        .into_variant()
    });

    for _ in 0..5 {
        sent_packets.expect("MovePlayerPos", |p| {
            matches!(p, ServerboundGamePacket::MovePlayerPos(_))
        });
        sent_packets.expect_tick_end();
        sent_packets.expect_empty();
        simulation.tick();
    }

    // mine the block again to make sure that it takes the same number of ticks
    simulation.write_message(StartMiningBlockEvent {
        entity: simulation.entity,
        position: pos2,
        force: false,
    });
    for _ in 0..150 {
        simulation.tick();
    }
    sent_packets.clear();
    simulation.tick();

    sent_packets.expect("PlayerAction { action: StopDestroyBlock }", |p| {
        matches!(
            p,
            ServerboundGamePacket::PlayerAction(p)
            if p.action == s_player_action::Action::StopDestroyBlock
        )
    });
}
