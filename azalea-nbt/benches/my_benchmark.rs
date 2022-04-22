use azalea_nbt::Tag;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use flate2::read::GzDecoder;
use std::{
    fs::File,
    io::{self, Cursor, Read, Seek, SeekFrom},
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
    // run Tag::read(&mut decoded_src_stream) asynchronously
    let nbt = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { Tag::read(&mut decoded_src_stream).await.unwrap() });

    let mut group = c.benchmark_group(filename);
    group.sample_size(1000);

    group.throughput(Throughput::Bytes(decoded_src.len() as u64));

    group.bench_function("Decode", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let mut owned_decoded_src_stream = decoded_src_stream.clone();
                owned_decoded_src_stream.seek(SeekFrom::Start(0)).unwrap();
                Tag::read(&mut owned_decoded_src_stream).await.unwrap();
            })
    });

    // group.bench_function("Encode", |b| {
    //     b.iter(|| {
    //         nbt.write(&mut io::sink()).unwrap();
    //     })
    // });
    group.finish();
}

fn bench(c: &mut Criterion) {
    bench_serialize("tests/bigtest.nbt", c);
    bench_serialize("tests/simple_player.dat", c);
    bench_serialize("tests/complex_player.dat", c);
    bench_serialize("tests/level.dat", c);
    bench_serialize("tests/stringtest.nbt", c);
    bench_serialize("tests/inttest.nbt", c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
