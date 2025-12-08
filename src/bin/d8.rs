use aoc2025::d8;

const FLNAME: &str = "inputs/d8.txt";
const P1_SPAN_EDGES: usize = 1000;

fn main() {
	let data = d8::load(FLNAME).unwrap();
	// println!("{:?}", data);
	// println!("part 1 {:?}", d8::p1(&data.clone(), P1_SPAN_EDGES));
	println!("part 2 {:?}", d8::p2(&data));
}
