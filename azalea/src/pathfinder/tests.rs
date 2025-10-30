use std::{
    collections::HashSet,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use azalea_block::BlockState;
use azalea_core::position::{BlockPos, ChunkPos, Vec3};
use azalea_world::{Chunk, ChunkStorage, PartialChunkStorage};

use super::{
    GotoEvent,
    astar::PathfinderTimeout,
    goals::BlockPosGoal,
    moves,
    simulation::{SimulatedPlayerBundle, Simulation},
};
use crate::pathfinder::goto_event::PathfinderOpts;

fn setup_blockposgoal_simulation(
    partial_chunks: &mut PartialChunkStorage,
    start_pos: BlockPos,
    end_pos: BlockPos,
    solid_blocks: &[BlockPos],
) -> Simulation {
    let mut simulation = setup_simulation_world(partial_chunks, start_pos, solid_blocks, &[]);

    // you can uncomment this while debugging tests to get trace logs
    // simulation.app.add_plugins(bevy_log::LogPlugin {
    //     level: bevy_log::Level::TRACE,
    //     filter: "".to_string(),
    //     ..Default::default()
    // });

    simulation.app.world_mut().write_message(GotoEvent {
        entity: simulation.entity,
        goal: Arc::new(BlockPosGoal(end_pos)),
        opts: PathfinderOpts {
            successors_fn: moves::default_move,
            allow_mining: false,
            retry_on_no_path: true,
            min_timeout: PathfinderTimeout::Nodes(1_000_000),
            max_timeout: PathfinderTimeout::Nodes(5_000_000),
        },
    });
    simulation
}

fn setup_simulation_world(
    partial_chunks: &mut PartialChunkStorage,
    start_pos: BlockPos,
    solid_blocks: &[BlockPos],
    extra_blocks: &[(BlockPos, BlockState)],
) -> Simulation {
    let mut chunk_positions = HashSet::new();
    for block_pos in solid_blocks {
        chunk_positions.insert(ChunkPos::from(block_pos));
    }
    for (block_pos, _) in extra_blocks {
        chunk_positions.insert(ChunkPos::from(block_pos));
    }

    let mut chunks = ChunkStorage::default();
    for chunk_pos in chunk_positions {
        partial_chunks.set(&chunk_pos, Some(Chunk::default()), &mut chunks);
    }
    for block_pos in solid_blocks {
        chunks.set_block_state(*block_pos, azalea_registry::Block::Stone.into());
    }
    for (block_pos, block_state) in extra_blocks {
        chunks.set_block_state(*block_pos, *block_state);
    }

    let player = SimulatedPlayerBundle::new(Vec3::new(
        start_pos.x as f64 + 0.5,
        start_pos.y as f64,
        start_pos.z as f64 + 0.5,
    ));
    Simulation::new(chunks, player)
}

pub fn assert_simulation_reaches(simulation: &mut Simulation, ticks: usize, end_pos: BlockPos) {
    wait_until_bot_starts_moving(simulation);
    for _ in 0..ticks {
        simulation.tick();
    }
    assert_eq!(BlockPos::from(simulation.position()), end_pos);
}

pub fn wait_until_bot_starts_moving(simulation: &mut Simulation) {
    let start_pos = simulation.position();
    let start_time = Instant::now();
    while simulation.position() == start_pos
        && !simulation.is_mining()
        && start_time.elapsed() < Duration::from_millis(500)
    {
        simulation.tick();
        thread::yield_now();
    }
}

#[test]
fn test_simple_forward() {
    let mut partial_chunks = PartialChunkStorage::default();
    let mut simulation = setup_blockposgoal_simulation(
        &mut partial_chunks,
        BlockPos::new(0, 71, 0),
        BlockPos::new(0, 71, 1),
        &[BlockPos::new(0, 70, 0), BlockPos::new(0, 70, 1)],
    );
    assert_simulation_reaches(&mut simulation, 20, BlockPos::new(0, 71, 1));
}

#[test]
fn test_double_diagonal_with_walls() {
    let mut partial_chunks = PartialChunkStorage::default();
    let mut simulation = setup_blockposgoal_simulation(
        &mut partial_chunks,
        BlockPos::new(0, 71, 0),
        BlockPos::new(2, 71, 2),
        &[
            BlockPos::new(0, 70, 0),
            BlockPos::new(1, 70, 1),
            BlockPos::new(2, 70, 2),
            BlockPos::new(1, 72, 0),
            BlockPos::new(2, 72, 1),
        ],
    );
    assert_simulation_reaches(&mut simulation, 30, BlockPos::new(2, 71, 2));
}

#[test]
fn test_jump_with_sideways_momentum() {
    let mut partial_chunks = PartialChunkStorage::default();
    let mut simulation = setup_blockposgoal_simulation(
        &mut partial_chunks,
        BlockPos::new(0, 71, 3),
        BlockPos::new(5, 76, 0),
        &[
            BlockPos::new(0, 70, 3),
            BlockPos::new(0, 70, 2),
            BlockPos::new(0, 70, 1),
            BlockPos::new(0, 70, 0),
            BlockPos::new(1, 71, 0),
            BlockPos::new(2, 72, 0),
            BlockPos::new(3, 73, 0),
            BlockPos::new(4, 74, 0),
            BlockPos::new(5, 75, 0),
        ],
    );
    assert_simulation_reaches(&mut simulation, 120, BlockPos::new(5, 76, 0));
}

#[test]
fn test_parkour_2_block_gap() {
    let mut partial_chunks = PartialChunkStorage::default();
    let mut simulation = setup_blockposgoal_simulation(
        &mut partial_chunks,
        BlockPos::new(0, 71, 0),
        BlockPos::new(0, 71, 3),
        &[BlockPos::new(0, 70, 0), BlockPos::new(0, 70, 3)],
    );
    assert_simulation_reaches(&mut simulation, 40, BlockPos::new(0, 71, 3));
}

#[test]
fn test_descend_and_parkour_2_block_gap() {
    let mut partial_chunks = PartialChunkStorage::default();
    let mut simulation = setup_blockposgoal_simulation(
        &mut partial_chunks,
        BlockPos::new(0, 71, 0),
        BlockPos::new(3, 67, 4),
        &[
            BlockPos::new(0, 70, 0),
            BlockPos::new(0, 69, 1),
            BlockPos::new(0, 68, 2),
            BlockPos::new(0, 67, 3),
            BlockPos::new(0, 66, 4),
            BlockPos::new(3, 66, 4),
        ],
    );
    assert_simulation_reaches(&mut simulation, 100, BlockPos::new(3, 67, 4));
}

#[test]
fn test_small_descend_and_parkour_2_block_gap() {
    let mut partial_chunks = PartialChunkStorage::default();
    let mut simulation = setup_blockposgoal_simulation(
        &mut partial_chunks,
        BlockPos::new(0, 71, 0),
        BlockPos::new(0, 70, 5),
        &[
            BlockPos::new(0, 70, 0),
            BlockPos::new(0, 70, 1),
            BlockPos::new(0, 69, 2),
            BlockPos::new(0, 69, 5),
        ],
    );
    assert_simulation_reaches(&mut simulation, 40, BlockPos::new(0, 70, 5));
}

#[test]
fn test_quickly_descend() {
    let mut partial_chunks = PartialChunkStorage::default();
    let mut simulation = setup_blockposgoal_simulation(
        &mut partial_chunks,
        BlockPos::new(0, 71, 0),
        BlockPos::new(0, 68, 3),
        &[
            BlockPos::new(0, 70, 0),
            BlockPos::new(0, 69, 1),
            BlockPos::new(0, 68, 2),
            BlockPos::new(0, 67, 3),
        ],
    );
    assert_simulation_reaches(&mut simulation, 60, BlockPos::new(0, 68, 3));
}

#[test]
fn test_2_gap_ascend_thrice() {
    let mut partial_chunks = PartialChunkStorage::default();
    let mut simulation = setup_blockposgoal_simulation(
        &mut partial_chunks,
        BlockPos::new(0, 71, 0),
        BlockPos::new(3, 74, 0),
        &[
            BlockPos::new(0, 70, 0),
            BlockPos::new(0, 71, 3),
            BlockPos::new(3, 72, 3),
            BlockPos::new(3, 73, 0),
        ],
    );
    assert_simulation_reaches(&mut simulation, 60, BlockPos::new(3, 74, 0));
}

#[test]
fn test_consecutive_3_gap_parkour() {
    let mut partial_chunks = PartialChunkStorage::default();
    let mut simulation = setup_blockposgoal_simulation(
        &mut partial_chunks,
        BlockPos::new(0, 71, 0),
        BlockPos::new(4, 71, 12),
        &[
            BlockPos::new(0, 70, 0),
            BlockPos::new(0, 70, 4),
            BlockPos::new(0, 70, 8),
            BlockPos::new(0, 70, 12),
            BlockPos::new(4, 70, 12),
        ],
    );
    assert_simulation_reaches(&mut simulation, 80, BlockPos::new(4, 71, 12));
}

#[test]
fn test_jumps_with_more_sideways_momentum() {
    let mut partial_chunks = PartialChunkStorage::default();
    let mut simulation = setup_blockposgoal_simulation(
        &mut partial_chunks,
        BlockPos::new(0, 71, 0),
        BlockPos::new(4, 74, 9),
        &[
            BlockPos::new(0, 70, 0),
            BlockPos::new(0, 70, 1),
            BlockPos::new(0, 70, 2),
            BlockPos::new(0, 71, 3),
            BlockPos::new(0, 72, 6),
            BlockPos::new(0, 73, 9),
            // this is the point where the bot might fall if it has too much momentum
            BlockPos::new(2, 73, 9),
            BlockPos::new(4, 73, 9),
        ],
    );
    assert_simulation_reaches(&mut simulation, 80, BlockPos::new(4, 74, 9));
}

#[test]
fn test_mine_through_non_colliding_block() {
    let mut partial_chunks = PartialChunkStorage::default();

    let mut simulation = setup_simulation_world(
        &mut partial_chunks,
        // the pathfinder can't actually dig straight down, so we start a block to the side so
        // it can descend correctly
        BlockPos::new(0, 72, 1),
        &[BlockPos::new(0, 71, 1)],
        &[
            (
                BlockPos::new(0, 71, 0),
                azalea_registry::Block::SculkVein.into(),
            ),
            (
                BlockPos::new(0, 70, 0),
                azalea_registry::Block::GrassBlock.into(),
            ),
            // this is an extra check to make sure that we don't accidentally break the block
            // below (since tnt will break instantly)
            (BlockPos::new(0, 69, 0), azalea_registry::Block::Tnt.into()),
        ],
    );

    simulation.app.world_mut().write_message(GotoEvent {
        entity: simulation.entity,
        goal: Arc::new(BlockPosGoal(BlockPos::new(0, 69, 0))),
        opts: PathfinderOpts::new()
            .min_timeout(PathfinderTimeout::Nodes(1_000_000))
            .max_timeout(PathfinderTimeout::Nodes(5_000_000)),
    });

    assert_simulation_reaches(&mut simulation, 200, BlockPos::new(0, 70, 0));
}
