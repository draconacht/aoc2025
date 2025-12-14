use aoc2025::d06;

const FLNAME: &str = "inputs/d6.txt";

fn main() {
	let operations = d06::load(FLNAME).unwrap();
	println!("{:?}", operations);
	println!("{:?}", d06::p1(operations.clone()));
	let ops2 = d06::load_p2(FLNAME).unwrap();
	println!("{:?}", ops2);
	println!("{:?}", d06::p2(ops2));
}
