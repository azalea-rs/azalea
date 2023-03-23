use azalea_nbt::Tag;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use flate2::read::GzDecoder;
use std::{
    fs::File,
    io::{self, Cursor, Read},
};

fn bench_file(filename: &str, c: &mut Criterion) {
    let mut file = File::open(filename).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mut src = &contents[..];

    // decode the original src so most of the time isn't spent on unzipping
    let mut decoded_src_decoder = GzDecoder::new(&mut src);
    let mut decoded_src = Vec::new();
    decoded_src_decoder.read_to_end(&mut decoded_src).unwrap();

    let mut decoded_src_stream = Cursor::new(&decoded_src[..]);

    let nbt = Tag::read(&mut decoded_src_stream).unwrap();
    decoded_src_stream.set_position(0);

    let mut group = c.benchmark_group(filename);

    group.throughput(Throughput::Bytes(decoded_src.len() as u64));

    group.bench_function("Decode", |b| {
        b.iter(|| {
            black_box(Tag::read(&mut decoded_src_stream).unwrap());
            decoded_src_stream.set_position(0);
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
    bench_file("tests/bigtest.nbt", c);
    bench_file("tests/simple_player.dat", c);
    bench_file("tests/complex_player.dat", c);
    bench_file("tests/level.dat", c);
    bench_file("tests/stringtest.nbt", c);
    bench_file("tests/inttest.nbt", c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
