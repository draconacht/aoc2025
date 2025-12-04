use aoc2025::d3;

const FLNAME: &str = "inputs/d3_sample.txt";

fn main() {
	let bats = d3::load(FLNAME).unwrap();
	println!("data: {:?}", bats.first_chunk::<3>());
	println!("part 1: {}", d3::p1(&bats));
	println!("part 2: {}", d3::p2_slower(&bats));
}
