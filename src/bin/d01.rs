use aoc2025::d01;

fn main() {
	let dirs = d01::load("inputs/d01.txt").unwrap();
	// println!("preview: {:?}", dirs.first_chunk::<10>());
	d01::vis(&dirs);
	println!("part 1: {:?}", d01::p1(&dirs));
	println!("part 2: {:?}", d01::p2(&dirs));
}
