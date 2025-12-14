use aoc2025::d12;
use criterion::{Criterion, criterion_group, criterion_main};
use itertools::Itertools;

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 11 bench", |b| {
		b.iter(|| {
			let (bricks, puzzles) = d12::load("inputs/d12.txt").unwrap();
			let nontrivial_puzzles = puzzles
				.iter()
				.filter(|puzzle| puzzle.required_efficiency(&bricks) < 1.0)
				.collect_vec();
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
