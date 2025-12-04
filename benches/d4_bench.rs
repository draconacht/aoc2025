use aoc2025::d4;
use criterion::{Criterion, criterion_group, criterion_main};

const FLNAME: &str = "inputs/d4.txt";

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("day 4 bench", |b| {
		b.iter(|| {
			let maze = d4::load(FLNAME).unwrap();
			let m = maze.iter().map(|x| x.as_slice()).collect::<Vec<&[d4::Cell]>>();
			println!("part 1: {}", d4::p1(&m));
			println!("part 2: {}", d4::p2(&m));
		})
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
