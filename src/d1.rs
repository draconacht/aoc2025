use crate::util::errors::MyError;
use std::{fs::File, io::Read, str::FromStr};

const INPUT_FILE: &str = "inputs/d1.txt";

#[derive(Debug, PartialEq)]
enum Direction {
	L(u16),
	R(u16),
}

impl FromStr for Direction {
	type Err = MyError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.chars().nth(0) {
			Some('L') => Ok(Direction::L(u16::from_str(&s[1..])?)),
			Some('R') => Ok(Direction::R(u16::from_str(&s[1..])?)),
			Some(_) => Err("invalid char")?,
			None => Err("stop")?,
		}
	}
}

#[derive(Debug, Clone)]
struct S {
	angle: i16,
	zeros: u16,
}

fn part1(dirs: &Vec<Direction>) -> Result<u16, MyError> {
	// PART 1
	let p1 = dirs.iter().fold(
		S {
			angle: 50,
			zeros: 0,
		},
		|mut s, next_dir| {
			match *next_dir {
				Direction::L(x) => s.angle = (s.angle - (x % 100) as i16).rem_euclid(100),
				Direction::R(x) => s.angle = (s.angle + (x % 100) as i16).rem_euclid(100),
			};
			if s.angle == 0 {
				s.zeros += 1
			}
			// println!("{:?}", s);
			s
		},
	);
	Ok(p1.zeros)
}

fn part2(dirs: &Vec<Direction>) -> Result<u16, MyError> {
	// PART 2
	let p2 = dirs.iter().fold(
		S {
			angle: 50,
			zeros: 0,
		},
		|s, next_dir| {
			let mut next = s.clone();
			match *next_dir {
				Direction::L(x) => {
					next.angle = (s.angle - (x % 100) as i16).rem_euclid(100);
					next.zeros += ((100 - s.angle + (x % 100) as i16) / 100) as u16 + (x / 100);
					if s.angle == 0 {
						next.zeros -= 1
					}
				}
				Direction::R(x) => {
					next.angle = (s.angle + (x % 100) as i16).rem_euclid(100);
					next.zeros += ((s.angle + (x % 100) as i16) / 100) as u16 + (x / 100);
				}
			};
			// println!("{:?}", next);
			next
		},
	);
	Ok(p2.zeros)
}

pub fn run() -> Result<(), MyError> {
	// LOAD INPUT
	let mut fl = File::open(INPUT_FILE)?;
	let mut s = String::new();
	fl.read_to_string(&mut s)?;

	let dirs = s
		.lines()
		.map(Direction::from_str)
		.collect::<Result<Vec<_>, _>>()?;

	println!("preview: {:?}", dirs.first_chunk::<10>());
	println!("part 1: {:?}", part1(&dirs)?);
	println!("part 2: {:?}", part2(&dirs)?);

	Ok(())
}
