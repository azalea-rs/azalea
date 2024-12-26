use std::{hint::black_box, sync::Arc, time::Duration};

use azalea::{
    pathfinder::{
        astar::{self, a_star, PathfinderTimeout},
        goals::{BlockPosGoal, Goal},
        mining::MiningCache,
        rel_block_pos::RelBlockPos,
        world::CachedWorld,
    },
    BlockPos,
};
use azalea_core::position::{ChunkBlockPos, ChunkPos};
use azalea_inventory::Menu;
use azalea_world::{Chunk, ChunkStorage, PartialChunkStorage};
use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use parking_lot::RwLock;
use rand::{rngs::StdRng, Rng, SeedableRng};

#[allow(dead_code)]
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

    let mut rng = StdRng::seed_from_u64(0);

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
    // move start down until it's on a solid block
    while chunks.get_block_state(&start).unwrap().is_air() {
        start = start.down(1);
    }
    start = start.up(1);

    let mut end = BlockPos::new(63, 4, 63);
    // move end down until it's on a solid block
    while chunks.get_block_state(&end).unwrap().is_air() {
        end = end.down(1);
    }
    end = end.up(1);

    (chunks, start, end)
}

fn generate_mining_world(
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

    // let mut rng = StdRng::seed_from_u64(0);

    for chunk_x in -size..size {
        for chunk_z in -size..size {
            let chunk_pos = ChunkPos::new(chunk_x, chunk_z);
            let chunk = chunks.get(&chunk_pos).unwrap();
            let mut chunk = chunk.write();
            for y in chunks.min_y..(chunks.min_y + chunks.height as i32) {
                for x in 0..16_u8 {
                    for z in 0..16_u8 {
                        chunk.set(
                            &ChunkBlockPos::new(x, y, z),
                            azalea_registry::Block::Stone.into(),
                            chunks.min_y,
                        );
                    }
                }
            }
        }
    }

    let start = BlockPos::new(-64, 4, -64);
    let end = BlockPos::new(0, 4, 0);

    (chunks, start, end)
}

fn run_pathfinder_benchmark(
    b: &mut Bencher<'_>,
    generate_world: fn(&mut PartialChunkStorage, u32) -> (ChunkStorage, BlockPos, BlockPos),
) {
    let mut partial_chunks = PartialChunkStorage::new(32);
    let successors_fn = azalea::pathfinder::moves::default_move;

    let (world, start, end) = generate_world(&mut partial_chunks, 4);

    let origin = start;

    b.iter(|| {
        let cached_world = CachedWorld::new(Arc::new(RwLock::new(world.clone().into())), origin);
        let mining_cache =
            MiningCache::new(Some(Menu::Player(azalea_inventory::Player::default())));
        let goal = BlockPosGoal(end);

        let successors = |pos: RelBlockPos| {
            azalea::pathfinder::call_successors_fn(&cached_world, &mining_cache, successors_fn, pos)
        };

        let astar::Path {
            movements,
            is_partial: partial,
        } = a_star(
            RelBlockPos::get_origin(origin),
            |n| goal.heuristic(n.apply(origin)),
            successors,
            |n| goal.success(n.apply(origin)),
            PathfinderTimeout::Time(Duration::MAX),
            PathfinderTimeout::Time(Duration::MAX),
        );

        assert!(!partial);

        black_box((movements, partial));
    })
}

fn bench_pathfinder(c: &mut Criterion) {
    // c.bench_function("bedrock", |b| {
    //     run_pathfinder_benchmark(b, generate_bedrock_world);
    // });
    let mut slow_group = c.benchmark_group("slow");
    slow_group.sample_size(10);
    slow_group.bench_function("mining", |b| {
        run_pathfinder_benchmark(b, generate_mining_world);
    });
    slow_group.finish();
}

criterion_group!(benches, bench_pathfinder);
criterion_main!(benches);
