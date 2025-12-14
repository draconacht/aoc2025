use aoc2025::d05;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d5.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 5 bench", |b| {
		b.iter(|| {
			let (haystacks, needles) = d05::load(FLNAME).unwrap();
			d05::p1(&haystacks, &needles);
			d05::p2(&haystacks);
		});
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
