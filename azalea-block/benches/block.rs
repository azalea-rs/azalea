use std::hint::black_box;

use azalea_block::{BlockState, BlockTrait};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

fn bench_block(c: &mut Criterion) {
    c.bench_function("Box<dyn BlockTrait> from BlockState", |b| {
        b.iter_batched(
            || {
                let mut blocks = Vec::with_capacity(1024);
                for _ in 0..1024 {
                    let id = rand::random_range(0..=BlockState::MAX_STATE);
                    let block = BlockState::try_from(id).unwrap();
                    blocks.push(block);
                }
                blocks
            },
            |blocks| {
                for block in blocks {
                    black_box(block.boxed());
                }
            },
            BatchSize::LargeInput,
        );
    });

    c.bench_function("&dyn BlockTrait from BlockState", |b| {
        b.iter_batched(
            || {
                let mut blocks = Vec::with_capacity(1024);
                for _ in 0..1024 {
                    let id = rand::random_range(0..=BlockState::MAX_STATE);
                    let block = BlockState::try_from(id).unwrap();
                    blocks.push(block);
                }
                blocks
            },
            |blocks| {
                for block in blocks {
                    let r: &dyn BlockTrait = block.to_trait();
                    black_box(r);
                }
            },
            BatchSize::LargeInput,
        );
    });
}

criterion_group!(benches, bench_block);
criterion_main!(benches);
