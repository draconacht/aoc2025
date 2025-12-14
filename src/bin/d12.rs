use aoc2025::d12;
use itertools::Itertools;

fn main() {
	let (bricks, puzzles) = d12::load("inputs/d12.txt").unwrap();
	let nontrivial_puzzles = puzzles
		.iter()
		.filter(|puzzle| puzzle.required_efficiency(&bricks) < 1.0)
		.collect_vec();
	println!("{}", nontrivial_puzzles.len());
}
