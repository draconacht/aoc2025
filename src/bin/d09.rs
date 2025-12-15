use aoc2025::d09;

fn main() {
	let data = d09::load("inputs/d9.txt").unwrap();
	println!("{:?}", data);
	println!("{:?}", d09::p1(&data));
	println!("{:?}", d09::p2(&data));
}
