use aoc2025::d11;
use criterion::{Criterion, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 11 bench", |b| {
		b.iter(|| {
			let inputs = d11::load("inputs/d10.txt").unwrap();
			d11::p1(inputs.clone());
			d11::p2(inputs);
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
