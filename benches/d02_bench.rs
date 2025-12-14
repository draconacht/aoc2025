use aoc2025::d02;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d2.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 2 bench", |b| {
		b.iter(|| {
			let ranges = d02::load(FLNAME).unwrap();
			d02::p1(&ranges);
			d02::p2(&ranges);
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
