//! Realistic benchmark what can load custom maps (currently block params and
//! entities not supported) How use:
//! For pathfinder start point place command block, after this bot will spawn on
//! command block. For pathfinder end point place diamond block.

use std::{fs, hint::black_box, path::Path, str::FromStr, sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use azalea::{
    BlockPos,
    pathfinder::{
        astar::{self, PathfinderTimeout, a_star},
        custom_state::CustomPathfinderStateRef,
        goals::{BlockPosGoal, Goal},
        mining::MiningCache,
        rel_block_pos::RelBlockPos,
        world::CachedWorld,
    },
};
use azalea_block::BlockState;
use azalea_core::position::ChunkPos;
use azalea_inventory::Menu;
use azalea_registry::{Block, Registry};
use azalea_world::{Chunk, ChunkStorage, PartialChunkStorage};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use mcdata::GenericBlockState;
use parking_lot::RwLock;
use rustmatica::Litematic;

fn load_litematic(
    path: impl AsRef<Path>,
    partial_chunks: &mut PartialChunkStorage,
    size: u32,
) -> Result<(ChunkStorage, BlockPos, BlockPos)> {
    let size = size as i32;
    let mut chunks = ChunkStorage::default();

    for chunk_x in -size..size {
        for chunk_z in -size..size {
            let chunk_pos = ChunkPos::new(chunk_x, chunk_z);
            partial_chunks.set(&chunk_pos, Some(Chunk::default()), &mut chunks);
        }
    }

    let (mut start, mut end) = (BlockPos::new(0, 0, 0), BlockPos::new(0, 0, 0));
    let litematic: Litematic<GenericBlockState> = Litematic::read_file(path)?;

    for region in litematic.regions {
        for (p, state) in region.blocks() {
            let p = BlockPos::new(p.x, p.y, p.z);
            let block = Block::from_str(&state.name).map_err(|e| anyhow!("{e}"))?;

            match block {
                Block::CommandBlock => start = p.up(1),
                Block::DiamondBlock => end = p.up(1),
                _ => {}
            }

            chunks.set_block_state(
                BlockPos::new(p.x, p.y, p.z),
                BlockState::try_from(block.to_u32()).map_err(|_| anyhow!("BlockState invalid"))?,
            );
        }
    }

    Ok((chunks, start, end))
}

fn bench_pathfinder(c: &mut Criterion) {
    bench_folder(c, "benches/assets/simple", false);
    bench_folder(c, "benches/assets/mine", true);
}

fn bench_folder(c: &mut Criterion, folder: &str, mine: bool) {
    let mut g = c.benchmark_group("realistic benches");
    let c = g.measurement_time(Duration::from_secs(20));

    let mut entries = vec![];
    for file in fs::read_dir(folder).unwrap() {
        let path = file.unwrap().path();
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".litematic")
            .to_string();
        entries.push((path, name));
    }

    for (path, name) in entries {
        let mut partial_chunks = PartialChunkStorage::new(32);
        let (world, start, end) = load_litematic(path, &mut partial_chunks, 16).unwrap();
        println!("Bench: {name}, Start: {start}, End: {end}");

        let (cached_world, mining_cache) = setup(&world, start, mine);
        if iteration(cached_world, mining_cache, start, end) {
            println!(
                "Partial path getted, its a bug. It maybe can works in non-test enviroment, but its must be fixed."
            );
            continue;
        }

        c.bench_function(&name, |b| {
            b.iter_batched(
                || setup(&world, start, mine),
                |(cached_world, mining_cache)| {
                    iteration(cached_world, mining_cache, start, end);
                },
                BatchSize::SmallInput,
            );
        });
    }
}

fn setup(world: &ChunkStorage, start: BlockPos, mine: bool) -> (CachedWorld, MiningCache) {
    let cached_world = CachedWorld::new(Arc::new(RwLock::new(world.clone().into())), start);
    let mining_cache = MiningCache::new(if mine {
        Some(Menu::Player(azalea_inventory::Player::default()))
    } else {
        None
    });
    (cached_world, mining_cache)
}

fn iteration(
    cached_world: CachedWorld,
    mining_cache: MiningCache,
    start: BlockPos,
    end: BlockPos,
) -> bool {
    let goal = BlockPosGoal(end);

    let successors = |pos: RelBlockPos| {
        azalea::pathfinder::call_successors_fn(
            &cached_world,
            &mining_cache,
            &CustomPathfinderStateRef::default(),
            azalea::pathfinder::moves::default_move,
            pos,
        )
    };

    let astar::Path {
        movements,
        is_partial: partial,
    } = a_star(
        RelBlockPos::get_origin(start),
        |n| goal.heuristic(n.apply(start)),
        successors,
        |n| goal.success(n.apply(start)),
        PathfinderTimeout::Time(Duration::MAX),
        PathfinderTimeout::Time(Duration::MAX),
    );

    black_box((movements, partial));

    partial
}

criterion_group!(benches, bench_pathfinder);
criterion_main!(benches);
