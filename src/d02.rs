use std::{
	fs::{File, read_to_string},
	io::Read,
	path::Path,
	str::FromStr,
};

use crate::util::errors::MyError;

#[derive(Debug)]
pub struct Range(u64, u64);

impl FromStr for Range {
	type Err = MyError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (start, end) = s.split_once("-").ok_or("invalid range")?;
		Ok(Range(u64::from_str(start)?, u64::from_str(end)? + 1))
	}
}

pub fn load(fname: impl AsRef<Path>) -> Result<Vec<Range>, MyError> {
	read_to_string(fname)?.trim().split(",").map(Range::from_str).collect()
}

pub fn p1_invalid_id(inp: &u64) -> bool {
	let inp_str = inp.to_string();
	inp_str[..inp_str.len() / 2] == inp_str[inp_str.len() / 2..]
}

pub fn p1(ranges: &[Range]) -> u64 {
	ranges
		.iter()
		.map(|Range(x, y)| (*x..*y).filter(p1_invalid_id).sum::<u64>())
		.sum()
}

pub fn p2_invalid_id(inp: &u64) -> bool {
	let inp_str = inp.to_string();
	(0..=(inp_str.len() / 2)).any(|chunk_size| {
		inp_str.len().is_multiple_of(chunk_size)
			&& (0..(inp_str.len() / chunk_size))
				.into_iter()
				.all(|idx| inp_str[(idx * chunk_size)..((idx + 1) * chunk_size)] == inp_str[..chunk_size])
	})
}

pub fn p2(ranges: &[Range]) -> u64 {
	ranges
		.iter()
		.map(|Range(x, y)| (*x..*y).filter(p2_invalid_id).sum::<u64>())
		.sum()
}
