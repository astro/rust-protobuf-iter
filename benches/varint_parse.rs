extern crate protobuf_iter;
use protobuf_iter::{Packed, PackedVarint};

fn parse_varint(data: &[u8]) {
    let _ = PackedVarint::parse(data).unwrap();
}

#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_varint 1", |b| b.iter(|| parse_varint(&[0b0000_0001])));
    c.bench_function("parse_varint 300", |b| b.iter(|| parse_varint(&[0b1010_1100, 0b0000_0010])));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
