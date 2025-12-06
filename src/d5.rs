use std::{
	cmp::{max, min},
	fs::read_to_string,
	path::Path,
	str::FromStr,
};

use crate::util::errors::MyError;

#[derive(Debug, Clone)]
pub struct RangeSet(u64, u64);

impl RangeSet {
	fn len(&self) -> u64 {
		self.1 - self.0
	}

	fn intersection(&self, other: &Self) -> Option<Self> {
		let (intersection_start, intersection_end) = (max(self.0, other.0), min(self.1, other.1));
		if intersection_start >= intersection_end {
			None
		} else {
			Some(RangeSet(intersection_start, intersection_end))
		}
	}

	fn sub_len(&self, other: &Self) -> u64 {
		let toret = match self.intersection(other) {
			None => self.len(),
			Some(Self(x0, x1)) => (x0 - self.0) + (self.1 - x1),
		};
		// println!(
		// 	"{:?} | {:?} | {:?} | {:?} ",
		// 	self,
		// 	other,
		// 	self.intersection(other),
		// 	toret
		// );
		toret
	}
}

impl FromStr for RangeSet {
	type Err = MyError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (s, e) = s.split_once("-").ok_or("invalid range")?;
		Ok(Self(u64::from_str(s)?, u64::from_str(e)? + 1))
	}
}

fn naive_lookup(haystacks: &[RangeSet], needle: u64) -> bool {
	haystacks
		.iter()
		.any(|haystack| haystack.0 <= needle && needle < haystack.1)
}

pub fn load(flname: impl AsRef<Path>) -> Result<(Vec<RangeSet>, Vec<u64>), MyError> {
	let dt = read_to_string(flname)?;
	let (r, l) = dt.split_once("\n\n").ok_or("invalid input")?;
	let ranges = r.lines().map(RangeSet::from_str).collect::<Result<_, _>>()?;
	let lookups = l.lines().map(u64::from_str).collect::<Result<_, _>>()?;
	Ok((ranges, lookups))
}

pub fn p1(haystacks: &[RangeSet], needles: &[u64]) -> usize {
	needles
		.iter()
		.filter(|&&needle| naive_lookup(haystacks, needle))
		.count()
}

pub fn p2(haystacks: &[RangeSet]) -> u64 {
	let mut h = haystacks.to_vec();
	h.sort_by_key(|rng| rng.0);
	let mut itrt = (0, 0);
	h.iter().for_each(|x| {
		itrt.1 += x.sub_len(&RangeSet(0, itrt.0));
		itrt.0 = max(itrt.0, x.1);
	});
	itrt.1
}
