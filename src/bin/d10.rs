use aoc2025::d10;

fn main() {
	let inputs = d10::load("inputs/d10.txt").unwrap();
	println!("{}", d10::p1(&inputs));
	println!("{}", d10::p2(&inputs));
}
