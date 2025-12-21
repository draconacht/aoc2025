use aoc2025::d01;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d01.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 1 bench", |b| {
		b.iter(|| {
			let d = d01::load(FLNAME).unwrap();
			d01::p1(&d);
			d01::p2(&d);
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
