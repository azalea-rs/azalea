use azalea_nbt::Tag;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

fn bench_serialize(filename: &str, c: &mut Criterion) {
    let mut file = File::open(filename).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mut src = std::io::Cursor::new(&contents[..]);
    file.seek(SeekFrom::Start(0)).unwrap();
    let nbt = Tag::read_gzip(&mut file).unwrap();

    let mut group = c.benchmark_group(filename);
    group.throughput(Throughput::Bytes(contents.len() as u64));
    group.bench_function("Deserialize As Blob", |b| {
        b.iter(|| {
            src.seek(SeekFrom::Start(0)).unwrap();
            Tag::read_gzip(&mut src).unwrap();
        })
    });
    group.bench_function("Serialize As Blob", |b| {
        b.iter(|| {
            nbt.write(&mut io::sink()).unwrap();
        })
    });
    group.finish();
}

fn bench(c: &mut Criterion) {
    bench_serialize("tests/bigtest.nbt", c);
    // bench_serialize::<data::PlayerData>("tests/simple_player.dat", c);
    // bench_serialize::<data::PlayerData>("tests/complex_player.dat", c);
    // bench_serialize::<data::Level>("tests/level.dat", c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
