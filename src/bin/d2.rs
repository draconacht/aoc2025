use aoc2025::d2;

const FLNAME: &str = "inputs/d2.txt";

fn main() {
	let ranges = d2::load(FLNAME).unwrap();
	println!("part 1: {}", d2::p1(&ranges));
	println!("part 2: {}", d2::p2(&ranges));
}
