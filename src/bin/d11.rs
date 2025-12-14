use aoc2025::d11;

const FLNAME: &str = "inputs/d11.txt";

fn main() {
	let data = d11::load(FLNAME).unwrap();
	println!("{:?}", data);
	println!("part 1 {:?}", d11::p1(data.clone()));
	println!("part 2 {:?}", d11::p2(data));
}
