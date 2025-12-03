use aoc2025::d3;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d3.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 3 bench", |b| {
		b.iter(|| {
			let ranges = d3::load(FLNAME).unwrap();
			println!("part 1: {}", d3::p1(&ranges));
			println!("part 2: {}", d3::p2(&ranges));
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
