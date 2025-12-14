use aoc2025::d10;
use criterion::{Criterion, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 10 bench", |b| {
		b.iter(|| {
			let inputs = d10::load("inputs/d10.txt").unwrap();
			d10::p1(&inputs);
			d10::p2(&inputs);
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
