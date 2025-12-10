use std::hint::black_box;

use azalea::pathfinder::mining::MiningCache;
use azalea_registry::builtin::BlockKind;
use criterion::{Criterion, criterion_group, criterion_main};

fn benchmark(c: &mut Criterion) {
    let mining_cache = MiningCache::new(None);

    let stone = BlockKind::Stone.into();
    c.bench_function("is_liquid stone", |b| {
        b.iter(|| mining_cache.is_liquid(black_box(stone)));
    });

    let water = BlockKind::Water.into();
    c.bench_function("is_liquid water", |b| {
        b.iter(|| mining_cache.is_liquid(black_box(water)));
    });

    let lava = BlockKind::Lava.into();
    c.bench_function("is_liquid lava", |b| {
        b.iter(|| mining_cache.is_liquid(black_box(lava)));
    });

    let waterlogged_slab = azalea_block::blocks::OakSlab {
        kind: azalea_block::properties::Type::Bottom,
        waterlogged: true,
    }
    .into();
    c.bench_function("is_liquid waterlogged slab", |b| {
        b.iter(|| mining_cache.is_liquid(black_box(waterlogged_slab)));
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
