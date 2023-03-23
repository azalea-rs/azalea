use std::io::Cursor;

use azalea_buf::McBufReadable;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use nbt as hematite_nbt;
use quartz_nbt::io::Flavor;

pub fn nbt_parse_bigtest(c: &mut Criterion) {
    let original = include_bytes!("../tests/bigtest.nbt").to_vec();
    let mut original_stream = Cursor::new(original);
    let original_tag = azalea_nbt::Tag::read_gzip(&mut original_stream).unwrap();
    let mut input = Vec::new();
    original_tag.write(&mut input).unwrap();
    let input = input.as_slice();

    c.bench_function("azalea_parse_bigtest", |b| {
        b.iter(|| {
            let input = black_box(input);
            let nbt = azalea_nbt::Tag::read(&mut Cursor::new(input)).unwrap();
            black_box(nbt);
        })
    });

    c.bench_function("graphite_parse_bigtest", |b| {
        b.iter(|| {
            let input = black_box(input);
            let nbt = graphite_binary::nbt::decode::read(&mut &input[..]).unwrap();
            black_box(nbt);
        })
    });

    c.bench_function("valence_parse_bigtest", |b| {
        b.iter(|| {
            let input = black_box(input);
            let nbt = valence_nbt::from_binary_slice(&mut &input[..]).unwrap();
            black_box(nbt);
        })
    });

    // c.bench_function("hematite_parse_bigtest", |b| {
    //     b.iter(|| {
    //         let input = black_box(input);

    //         let cursor = Cursor::new(input);
    //         let blob: hematite_nbt::Blob =
    // hematite_nbt::from_reader(cursor).unwrap();         black_box(blob);
    //     })
    // });

    // c.bench_function("quartz_parse_bigtest", |b| {
    //     b.iter(|| {
    //         let input = black_box(input);

    //         let mut cursor = Cursor::new(input);
    //         let nbt = quartz_nbt::io::read_nbt(&mut cursor,
    // Flavor::Uncompressed).unwrap();         black_box(nbt);
    //     })
    // });
}

pub fn nbt_write_bigtest(c: &mut Criterion) {
    let original = include_bytes!("../tests/bigtest.nbt").to_vec();
    let mut original_stream = Cursor::new(original);
    let original_tag = azalea_nbt::Tag::read_gzip(&mut original_stream).unwrap();
    let mut input = Vec::new();
    original_tag.write(&mut input).unwrap();
    let input = input.as_slice();

    let mut cursor = Cursor::new(input);
    let nbt = azalea_nbt::Tag::read_from(&mut cursor).unwrap();
    c.bench_function("azalea_write_bigtest", |b| {
        b.iter(|| {
            let nbt = black_box(&nbt);
            let mut written = Vec::new();
            nbt.write(&mut written).unwrap();
            black_box(written);
        })
    });

    let nbt = graphite_binary::nbt::decode::read(&mut &input[..]).unwrap();
    c.bench_function("graphite_write_bigtest", |b| {
        b.iter(|| {
            let nbt = black_box(&nbt);
            let written = graphite_binary::nbt::encode::write(nbt);
            black_box(written);
        })
    });

    let nbt = valence_nbt::from_binary_slice(&mut &input[..]).unwrap();
    c.bench_function("valence_write_bigtest", |b| {
        b.iter(|| {
            let nbt = black_box(&nbt);
            let mut written = Vec::new();
            valence_nbt::to_binary_writer(&mut written, &nbt.0, &nbt.1).unwrap();
            black_box(written);
        })
    });

    // let cursor = Cursor::new(input);
    // let nbt: hematite_nbt::Blob = hematite_nbt::from_reader(cursor).unwrap();
    // c.bench_function("hematite_write_bigtest", |b| {
    //     b.iter(|| {
    //         let nbt = black_box(&nbt);
    //         let mut written = Vec::new();
    //         hematite_nbt::to_writer(&mut written, nbt, None).unwrap();
    //         black_box(written);
    //     })
    // });

    // let mut cursor = Cursor::new(input);
    // let (nbt, _) = quartz_nbt::io::read_nbt(&mut cursor,
    // Flavor::Uncompressed).unwrap(); c.bench_function("
    // quartz_write_bigtest", |b| {     b.iter(|| {
    //         let nbt = black_box(&nbt);
    //         let mut written = Vec::new();
    //         quartz_nbt::io::write_nbt(&mut written, None, nbt,
    // Flavor::Uncompressed).unwrap();         black_box(written);
    //     })
    // });
}

// criterion_group!(benches, nbt_write_bigtest);
criterion_group!(benches, nbt_parse_bigtest, nbt_write_bigtest);
criterion_main!(benches);
