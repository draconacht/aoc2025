use aoc2025::d6;

const FLNAME: &str = "inputs/d6.txt";

fn main() {
	let operations = d6::load(FLNAME).unwrap();
	println!("{:?}", operations);
	println!("{:?}", d6::p1(operations.clone()));
	let ops2 = d6::load_p2(FLNAME).unwrap();
	println!("{:?}", ops2);
	println!("{:?}", d6::p2(ops2));
}
