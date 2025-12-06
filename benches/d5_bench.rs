use aoc2025::d5;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d5.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 5 bench", |b| {
		b.iter(|| {
			let (haystacks, needles) = d5::load(FLNAME).unwrap();
			println!("part 1: {}", d5::p1(&haystacks, &needles));
			println!("part 2: {}", d5::p2(&haystacks));
		});
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
