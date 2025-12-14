use aoc2025::d01;
use criterion::{Criterion, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 1", |b| b.iter(d01::run));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
