use aoc2025::d03;

const FLNAME: &str = "inputs/d3.txt";

fn main() {
	let bats = d03::load(FLNAME).unwrap();
	// println!("data: {:?}", bats.first_chunk::<3>());
	println!("part 1: {}", d03::p1(&bats));
	println!("part 2: {}", d03::p2(&bats));
}
