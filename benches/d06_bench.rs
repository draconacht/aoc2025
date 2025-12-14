use aoc2025::d06;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d6.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 6 bench", |b| {
		b.iter(|| {
			let operations = d06::load(FLNAME).unwrap();
			d06::p1(operations.clone());
			let ops2 = d06::load_p2(FLNAME).unwrap();
			d06::p2(ops2);
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
