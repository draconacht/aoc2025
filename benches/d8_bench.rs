use aoc2025::d8;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d8.txt";
const P1_SPAN_EDGES: usize = 1000;

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 8 bench", |b| {
		b.iter(|| {
			let data = d8::load(FLNAME).unwrap();
			// d8::p1(&data.clone(), P1_SPAN_EDGES);
			d8::p2(&data);
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
