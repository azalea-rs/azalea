use std::hint::black_box;

use azalea_core::position::ChunkBlockPos;
use azalea_registry::builtin::BlockKind;
use azalea_world::{BitStorage, Chunk};
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_chunks(c: &mut Criterion) {
    c.bench_function("Chunk::set", |b| {
        b.iter(|| {
            let mut chunk = Chunk::default();

            for x in 0..16 {
                for z in 0..16 {
                    chunk.set_block_state(
                        &ChunkBlockPos::new(x, 1, z),
                        BlockKind::Bedrock.into(),
                        0,
                    );
                }
            }

            black_box(chunk);
        });
    });
}

fn bench_bitstorage(c: &mut Criterion) {
    c.bench_function("BitStorage::set", |b| {
        let mut storage = BitStorage::new(1, 4096, None).unwrap();
        b.iter(|| {
            storage.set(136, 1);
        });
        black_box(storage);
    });
}

criterion_group!(benches, bench_chunks, bench_bitstorage);
criterion_main!(benches);
