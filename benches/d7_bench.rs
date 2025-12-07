use aoc2025::d7;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d7.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 7 bench", |b| {
		b.iter(|| {
			let (start, mut data) = d7::load(FLNAME).unwrap();
			// println!("{:?}", data);
			println!("part 1 {:?}", d7::p1(start, &mut data.clone()));
			println!("part 2 {:?}", d7::p2(start, &mut data));
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
