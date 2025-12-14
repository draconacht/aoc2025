use aoc2025::d07;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d7.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 7 bench", |b| {
		b.iter(|| {
			let (start, mut data) = d07::load(FLNAME).unwrap();
			d07::p1(start, &mut data.clone());
			d07::p2(start, &mut data);
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
