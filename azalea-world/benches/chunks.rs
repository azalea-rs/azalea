use std::hint::black_box;

use azalea_core::position::ChunkBlockPos;
use azalea_registry::builtin::BlockKind;
use azalea_world::{BitStorage, Chunk};
use criterion::{BatchSize, Bencher, Criterion, criterion_group, criterion_main};

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

fn bench_bitstorage_with(b: &mut Bencher, bits: usize, size: usize) {
    let mut storage = BitStorage::new(bits, size, None).unwrap();
    b.iter_batched(
        || {
            // let index = rand
            let mut vec = Vec::with_capacity(size);
            for _ in 0..size {
                vec.push(rand::random_range(0..size));
            }
            vec
        },
        |indices| {
            for index in indices {
                storage.set(index, 1);
            }
        },
        BatchSize::SmallInput,
    );
    black_box(storage);
}

fn bench_bitstorage(c: &mut Criterion) {
    c.bench_function("BitStorage::set (1 bit per entry)", |b| {
        bench_bitstorage_with(b, 1, 4096)
    });
    c.bench_function("BitStorage::set (2 bits per entry)", |b| {
        bench_bitstorage_with(b, 2, 4096)
    });
    c.bench_function("BitStorage::set (3 bits per entry)", |b| {
        bench_bitstorage_with(b, 3, 4096)
    });
}

criterion_group!(benches, bench_chunks, bench_bitstorage);
criterion_main!(benches);
