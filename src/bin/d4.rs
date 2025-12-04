use aoc2025::d4;

const FLNAME: &str = "inputs/d4.txt";

fn main() {
	let mut maze = d4::load(FLNAME).unwrap();
	println!("part 1: {}", d4::p1(&mut maze.clone()));
	println!("part 2: {}", d4::p2(&mut maze));
}
