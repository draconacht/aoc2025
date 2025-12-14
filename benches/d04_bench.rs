use aoc2025::d04;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d4.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 4 bench", |b| {
		b.iter(|| {
			let mut maze = d04::load(FLNAME).unwrap();
			d04::p1(&mut maze.clone());
			d04::p2(&mut maze);
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
