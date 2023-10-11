use std::{hint::black_box, sync::Arc, time::Duration};

use azalea::{
    pathfinder::{
        astar::{self, a_star},
        goals::BlockPosGoal,
        mining::MiningCache,
        world::CachedWorld,
        Goal,
    },
    BlockPos,
};
use azalea_core::position::{ChunkBlockPos, ChunkPos};
use azalea_inventory::Menu;
use azalea_world::{Chunk, ChunkStorage, PartialChunkStorage};
use criterion::{criterion_group, criterion_main, Criterion};
use parking_lot::RwLock;
use rand::Rng;

fn generate_bedrock_world(
    partial_chunks: &mut PartialChunkStorage,
    size: u32,
) -> (ChunkStorage, BlockPos, BlockPos) {
    let size = size as i32;

    let mut chunks = ChunkStorage::default();
    for chunk_x in -size..size {
        for chunk_z in -size..size {
            let chunk_pos = ChunkPos::new(chunk_x, chunk_z);
            partial_chunks.set(&chunk_pos, Some(Chunk::default()), &mut chunks);
        }
    }

    let mut rng = rand::thread_rng();

    for chunk_x in -size..size {
        for chunk_z in -size..size {
            let chunk_pos = ChunkPos::new(chunk_x, chunk_z);
            let chunk = chunks.get(&chunk_pos).unwrap();
            let mut chunk = chunk.write();
            for x in 0..16_u8 {
                for z in 0..16_u8 {
                    chunk.set(
                        &ChunkBlockPos::new(x, 1, z),
                        azalea_registry::Block::Bedrock.into(),
                        chunks.min_y,
                    );
                    if rng.gen_bool(0.5) {
                        chunk.set(
                            &ChunkBlockPos::new(x, 2, z),
                            azalea_registry::Block::Bedrock.into(),
                            chunks.min_y,
                        );
                    }
                }
            }
        }
    }

    let mut start = BlockPos::new(-64, 4, -64);
    // move start down until it's on bedrock
    while chunks.get_block_state(&start).unwrap().is_air() {
        start = start.down(1);
    }
    start = start.up(1);

    let mut end = BlockPos::new(63, 4, 63);
    // move end down until it's on bedrock
    while chunks.get_block_state(&end).unwrap().is_air() {
        end = end.down(1);
    }
    end = end.up(1);

    (chunks, start, end)
}

fn bench_pathfinder(c: &mut Criterion) {
    c.bench_function("bedrock", |b| {
        let mut partial_chunks = PartialChunkStorage::new(32);
        let successors_fn = azalea::pathfinder::moves::default_move;

        b.iter(|| {
            let (world, start, end) = generate_bedrock_world(&mut partial_chunks, 4);
            let cached_world = CachedWorld::new(Arc::new(RwLock::new(world.into())));
            let mining_cache = MiningCache::new(Menu::Player(azalea_inventory::Player::default()));
            let goal = BlockPosGoal(end);

            let successors = |pos: BlockPos| {
                azalea::pathfinder::call_successors_fn(
                    &cached_world,
                    &mining_cache,
                    successors_fn,
                    pos,
                )
            };

            let astar::Path { movements, partial } = a_star(
                start,
                |n| goal.heuristic(n),
                successors,
                |n| goal.success(n),
                Duration::MAX,
            );

            black_box((movements, partial));
        })
    });
}

criterion_group!(benches, bench_pathfinder);
criterion_main!(benches);
