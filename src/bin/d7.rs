use aoc2025::d7;

const FLNAME: &str = "inputs/d7.txt";

fn main() {
	let (start, mut data) = d7::load(FLNAME).unwrap();
	println!("{:?}", data);
	println!("part 1 {:?}", d7::p1(start, &mut data.clone()));
	println!("part 2 {:?}", d7::p2(start, &mut data));
}
