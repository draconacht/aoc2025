use aoc2025::d4;

const FLNAME: &str = "inputs/d4.txt";

fn main() {
	let maze = d4::load(FLNAME).unwrap();
	let m = maze.iter().map(|x| x.as_slice()).collect::<Vec<&[d4::Cell]>>();
	println!("part 1: {}", d4::p1(&m));
	println!("part 2: {}", d4::p2(&m));
}
