use aoc2025::d2;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d2.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 2 bench", |b| {
		b.iter(|| {
			let ranges = d2::load(FLNAME).unwrap();
			println!("part 1: {}", d2::p1(&ranges));
			println!("part 2: {}", d2::p2(&ranges));
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
