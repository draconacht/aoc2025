use aoc2025::d04;

const FLNAME: &str = "inputs/d4.txt";

fn main() {
	let mut maze = d04::load(FLNAME).unwrap();
	println!("part 1: {}", d04::p1(&mut maze.clone()));
	println!("part 2: {}", d04::p2(&mut maze));
}
