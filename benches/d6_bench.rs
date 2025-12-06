use aoc2025::d6;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d6.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 6 bench", |b| {
		b.iter(|| {
			d6::load(FLNAME).unwrap();
			d6::load_p2(FLNAME).unwrap();
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
