use azalea_client::{
    inventory::SetSelectedHotbarSlotEvent, mining::StartMiningBlockEvent, test_utils::prelude::*,
};
use azalea_core::{
    direction::Direction,
    position::{BlockPos, ChunkPos, Vec3},
    resource_location::ResourceLocation,
};
use azalea_entity::LookDirection;
use azalea_protocol::{
    common::movements::{PositionMoveRotation, RelativeMovements},
    packets::{
        ConnectionProtocol, Packet,
        game::{
            ClientboundBlockUpdate, ClientboundPlayerPosition, ServerboundPlayerAction,
            ServerboundSetCarriedItem, ServerboundSwing, s_interact::InteractionHand,
            s_player_action,
        },
    },
};
use azalea_registry::{Block, DataRegistry, DimensionType};

#[test]
fn test_packet_order_set_carried_item() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    let sent_packets = SentPackets::new(&mut simulation);
    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(0),
        ResourceLocation::new("azalea:overworld"),
    ));

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.tick();

    let pos = BlockPos::new(0, 2, 0);
    simulation.receive_packet(ClientboundBlockUpdate {
        pos,
        block_state: Block::Stone.into(),
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
    assert_eq!(simulation.get_block_state(pos), Some(Block::Stone.into()));
    simulation.with_component_mut::<LookDirection>(|look| {
        // look down
        look.update_x_rot(90.);
    });

    simulation.tick();
    simulation.tick();
    simulation.tick();

    simulation.send_event(SetSelectedHotbarSlotEvent {
        entity: simulation.entity,
        slot: 1,
    });
    simulation.send_event(StartMiningBlockEvent {
        entity: simulation.entity,
        position: pos,
    });

    sent_packets.clear();
    simulation.tick();
    sent_packets.expect("ServerboundPlayerAction", |p| {
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
    sent_packets.expect("SetCarriedItem", |p| {
        p == &ServerboundSetCarriedItem { slot: 1 }.into_variant()
    });
    sent_packets.expect("Swing 2", |p| {
        p == &ServerboundSwing {
            hand: InteractionHand::MainHand,
        }
        .into_variant()
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    simulation.tick();

    sent_packets.expect("Swing", |p| {
        p == &ServerboundSwing {
            hand: InteractionHand::MainHand,
        }
        .into_variant()
    });
    sent_packets.expect_tick_end();
    sent_packets.expect_empty();
}
