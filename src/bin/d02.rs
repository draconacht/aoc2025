use aoc2025::d02;

const FLNAME: &str = "inputs/d2.txt";

fn main() {
	let ranges = d02::load(FLNAME).unwrap();
	println!("part 1: {}", d02::p1(&ranges));
	println!("part 2: {}", d02::p2(&ranges));
}
