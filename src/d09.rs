use itertools::Itertools;
use std::{fs::read_to_string, path::Path, str::FromStr};

use crate::util::errors::MyError;

#[derive(Debug, Clone)]
pub struct Pos(u32, u32);

impl FromStr for Pos {
	type Err = MyError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let pieces = s.split_once(",").ok_or("shit")?;
		Ok(Self(pieces.0.parse::<u32>()?, pieces.1.parse::<u32>()?))
	}
}

fn area(p0: &Pos, p1: &Pos) -> u64 {
	(p1.1.abs_diff(p0.1) + 1) as u64 * (p1.0.abs_diff(p0.0) + 1) as u64
}

fn sorted((p0, p1): (Pos, Pos)) -> (Pos, Pos) {
	(
		Pos(u32::min(p0.0, p1.0), u32::min(p0.1, p1.1)),
		Pos(u32::max(p0.0, p1.0), u32::max(p1.1, p0.1)),
	)
}

pub fn load(flname: impl AsRef<Path>) -> Result<Vec<Pos>, MyError> {
	read_to_string(flname)?.lines().map(Pos::from_str).try_collect()
}

pub fn p1(points: &[Pos]) -> u64 {
	let (a, b) = points
		.iter()
		.tuple_combinations::<(_, _)>()
		.max_by_key(|(a, b)| area(a, b))
		.ok_or("fuck")
		.unwrap();
	area(a, b)
}

pub fn p2(points: &[Pos]) -> u64 {
	let (a, b) = points
		.iter()
		.tuple_combinations::<(_, _)>()
		.map(|(a, b)| sorted((a.clone(), b.clone())))
		.sorted_by_key(|(a, b)| area(a, b))
		.rfind(|(a, b)| {
			points
				.iter()
				.tuple_windows::<(_, _)>() // TODO - wrap around
				.map(|(a, b)| sorted((a.clone(), b.clone())))
				.all(|(Pos(x0, y0), Pos(x1, y1))| !(x0 < b.0 && y0 < b.1 && x1 > a.0 && y1 > a.1))
		})
		.unwrap();
	println!("{:?} {:?}", a, b);
	area(&a, &b)
}
