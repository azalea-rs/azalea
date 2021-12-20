use azalea_nbt::Tag;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use flate2::read::GzDecoder;
use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

fn bench_serialize(filename: &str, c: &mut Criterion) {
    let mut file = File::open(filename).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mut src = std::io::Cursor::new(&contents[..]);

    // decode the original src so most of the time isn't spent on unzipping
    let mut decoded_src_decoder = GzDecoder::new(&mut src);
    let mut decoded_src = Vec::new();
    decoded_src_decoder.read_to_end(&mut decoded_src).unwrap();
    let mut decoded_src_stream = std::io::Cursor::new(decoded_src.clone());

    file.seek(SeekFrom::Start(0)).unwrap();
    let nbt = Tag::read_gzip(&mut file).unwrap();

    let mut group = c.benchmark_group(filename);
    group.throughput(Throughput::Bytes(decoded_src.len() as u64));
    group.bench_function("Decode", |b| {
        b.iter(|| {
            decoded_src_stream.seek(SeekFrom::Start(0)).unwrap();
            Tag::read(&mut decoded_src_stream).unwrap();
        })
    });
    group.bench_function("Encode", |b| {
        b.iter(|| {
            nbt.write(&mut io::sink()).unwrap();
        })
    });
    group.finish();
}

fn bench(c: &mut Criterion) {
    bench_serialize("tests/bigtest.nbt", c);
    // bench_serialize("tests/simple_player.dat", c);
    // bench_serialize("tests/complex_player.dat", c);
    bench_serialize("tests/level.dat", c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
