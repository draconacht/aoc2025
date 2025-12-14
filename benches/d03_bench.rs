use aoc2025::d03;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d3.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 3 bench", |b| {
		b.iter(|| {
			let ranges = d03::load(FLNAME).unwrap();
			d03::p1(&ranges);
			d03::p2_slower(&ranges);
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
