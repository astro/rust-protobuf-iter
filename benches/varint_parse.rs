extern crate protobuf_iter;
use protobuf_iter::{Packed, PackedVarint, PackedIter};

fn parse_varint(data: &[u8]) {
    let _ = PackedVarint::parse(data).unwrap();
}

#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_varint 1", |b| b.iter(|| parse_varint(&[0b0000_0001])));
    c.bench_function("parse_varint 300", |b| b.iter(|| parse_varint(&[0b1010_1100, 0b0000_0010])));

    let mut buf = Vec::<u8>::new();
    for i in 0..131073u32 {
        let mut n = i;
        let mut bytes = 0;
        while bytes < 1 || n != 0 {
            let b = n as u8 & 0b0111_1111;
            n >>= 7;
            let flag = if n != 0 {
                0b1000_0000
            } else {
                0
            };
            buf.push(flag | b);
            bytes += 1;
        }
    }
    c.bench_function("parse_varint iter", move |b| b.iter(
        || {
            let mut iter: PackedIter<'_, PackedVarint, u32> = PackedIter::new(&buf);
            for i in 0..131073u32 {
                assert_eq!(Some(i), iter.next());
            }
            assert_eq!(None, iter.next());
        }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
