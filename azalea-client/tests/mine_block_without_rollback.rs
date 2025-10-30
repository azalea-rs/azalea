use azalea_client::{mining::StartMiningBlockEvent, test_utils::prelude::*};
use azalea_core::position::{BlockPos, ChunkPos};
use azalea_protocol::packets::{
    ConnectionProtocol,
    game::{ClientboundBlockChangedAck, ClientboundBlockUpdate},
};
use azalea_registry::Block;

#[test]
fn test_mine_block_without_rollback() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    simulation.receive_packet(default_login_packet());

    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.tick();

    let pos = BlockPos::new(1, 2, 3);
    simulation.receive_packet(ClientboundBlockUpdate {
        pos,
        // tnt is used for this test because it's insta-mineable so we don't have to waste ticks
        // waiting
        block_state: Block::Tnt.into(),
    });
    simulation.tick();
    assert_eq!(simulation.get_block_state(pos), Some(Block::Tnt.into()));

    simulation.write_message(StartMiningBlockEvent {
        entity: simulation.entity,
        position: pos,
        force: true,
    });
    simulation.tick();
    assert_eq!(simulation.get_block_state(pos), Some(Block::Air.into()));

    // server acknowledged our change by sending a BlockUpdate + BlockChangedAck, so
    // no rollback
    simulation.receive_packet(ClientboundBlockUpdate {
        pos,
        block_state: Block::Air.into(),
    });
    simulation.receive_packet(ClientboundBlockChangedAck { seq: 1 });
    simulation.tick();
    assert_eq!(simulation.get_block_state(pos), Some(Block::Air.into()));
}
