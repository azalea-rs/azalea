use azalea_crypto::{create_cipher, encrypt_packet};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};

fn bench(c: &mut Criterion) {
    let (enc, dec) = create_cipher(b"0123456789abcdef");

    let mut packet = [0u8; 65536];
    for i in 0..packet.len() {
        packet[i] = i as u8;
    }

    c.bench_function("Encrypt", |b| {
        b.iter(|| encrypt_packet(&mut enc.clone(), &mut packet.clone()))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
