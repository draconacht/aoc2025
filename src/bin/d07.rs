use aoc2025::d07;

const FLNAME: &str = "inputs/d7.txt";

fn main() {
	let (start, mut data) = d07::load(FLNAME).unwrap();
	println!("{:?}", data);
	println!("part 1 {:?}", d07::p1(start, &mut data.clone()));
	println!("part 2 {:?}", d07::p2(start, &mut data));
}
