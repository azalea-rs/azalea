use azalea::{
    pathfinder::simulation::{SimulatedPlayerBundle, SimulationSet},
    Vec3,
};
use azalea_core::position::{ChunkBlockPos, ChunkPos};
use azalea_world::{Chunk, ChunkStorage, PartialChunkStorage};
use criterion::{criterion_group, criterion_main, Bencher, Criterion};

#[allow(dead_code)]
fn generate_world(partial_chunks: &mut PartialChunkStorage, size: u32) -> ChunkStorage {
    let size = size as i32;

    let mut chunks = ChunkStorage::default();
    for chunk_x in -size..size {
        for chunk_z in -size..size {
            let chunk_pos = ChunkPos::new(chunk_x, chunk_z);
            partial_chunks.set(&chunk_pos, Some(Chunk::default()), &mut chunks);
        }
    }

    for chunk_x in -size..size {
        for chunk_z in -size..size {
            let chunk_pos = ChunkPos::new(chunk_x, chunk_z);
            let chunk = chunks.get(&chunk_pos).unwrap();
            let mut chunk = chunk.write();
            for x in 0..16_u8 {
                for z in 0..16_u8 {
                    chunk.set(
                        &ChunkBlockPos::new(x, 1, z),
                        azalea_registry::Block::OakFence.into(),
                        chunks.min_y,
                    );
                }
            }
        }
    }

    // let mut start = BlockPos::new(-64, 4, -64);
    // // move start down until it's on a solid block
    // while chunks.get_block_state(&start).unwrap().is_air() {
    //     start = start.down(1);
    // }
    // start = start.up(1);

    // let mut end = BlockPos::new(63, 4, 63);
    // // move end down until it's on a solid block
    // while chunks.get_block_state(&end).unwrap().is_air() {
    //     end = end.down(1);
    // }
    // end = end.up(1);

    chunks
}

fn run_physics_benchmark(b: &mut Bencher<'_>) {
    let mut partial_chunks = PartialChunkStorage::new(32);

    let world = generate_world(&mut partial_chunks, 4);

    let mut simulation_set = SimulationSet::new(world);

    // let entity = simulation_set.spawn(SimulatedPlayerBundle::new(Vec3::new(0.0,
    // 4.0, 0.0))); for _ in 0..20 {
    //     simulation_set.tick();
    //     println!("tick over");
    // }
    // simulation_set.despawn(entity);
    // std::process::exit(0);

    b.iter(|| {
        let entity = simulation_set.spawn(SimulatedPlayerBundle::new(Vec3::new(0.5, 2.0, 0.5)));
        for _ in 0..20 {
            simulation_set.tick();
        }
        simulation_set.despawn(entity);
    })
}

fn bench_pathfinder(c: &mut Criterion) {
    c.bench_function("physics", |b| {
        run_physics_benchmark(b);
    });
}

criterion_group!(benches, bench_pathfinder);
criterion_main!(benches);
