use aoc2025::d5;

const FLNAME: &str = "inputs/d5.txt";

fn main() {
	let (haystacks, needles) = d5::load(FLNAME).unwrap();
	println!("part 1: {}", d5::p1(&haystacks, &needles));
	println!("part 2: {}", d5::p2(&haystacks));
}
