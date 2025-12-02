use std::{fs::File, io::Read, str::FromStr};

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

pub fn load(fname: &str) -> Result<Vec<Range>, MyError> {
	let mut fl = File::open(fname)?;
	let mut s = String::new();
	fl.read_to_string(&mut s)?;

	let pieces = s.trim().split(",");
	pieces.map(Range::from_str).collect()
}

pub fn p1_invalid_id(inp: &u64) -> bool {
	let inp_str = inp.to_string();
	inp_str[..inp_str.len() / 2] == inp_str[inp_str.len() / 2..]
}

pub fn p1(ranges: &Vec<Range>) -> u64 {
	let mut result: u64 = 0;
	for range in ranges.iter() {
		result += (range.0..range.1).filter(p1_invalid_id).sum::<u64>()
	}
	result
}

pub fn p2_invalid_id(inp: &u64) -> bool {
	let inp_str = inp.to_string();
	(0..=(inp_str.len() / 2)).any(|chunk_size| {
		inp_str.len().is_multiple_of(chunk_size)
			&& (0..(inp_str.len() / chunk_size)).into_iter().all(|idx| {
				inp_str[(idx * chunk_size)..((idx + 1) * chunk_size)] == inp_str[..chunk_size]
			})
	})
}

pub fn p2(ranges: &Vec<Range>) -> u64 {
	let mut result: u64 = 0;
	for range in ranges.iter() {
		result += (range.0..range.1).filter(p2_invalid_id).sum::<u64>()
	}
	result
}
