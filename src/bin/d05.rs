use aoc2025::d05;

const FLNAME: &str = "inputs/d5.txt";

fn main() {
	let (haystacks, needles) = d05::load(FLNAME).unwrap();
	println!("part 1: {}", d05::p1(&haystacks, &needles));
	println!("part 2: {}", d05::p2(&haystacks));
}
