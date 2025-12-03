use std::{cmp::max, fs::read_to_string, path::Path, str::FromStr};

use crate::util::errors::MyError;

#[derive(Debug)]
pub struct Battery {
	joltages: Vec<u8>,
}

impl FromStr for Battery {
	type Err = MyError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Battery {
			joltages: s
				.chars()
				.map(|ch| u8::from_str(&ch.to_string()))
				.collect::<Result<Vec<u8>, _>>()?,
		})
	}
}

pub fn load(flname: impl AsRef<Path>) -> Result<Vec<Battery>, MyError> {
	read_to_string(flname)?
		.trim()
		.lines()
		.map(Battery::from_str)
		.collect()
}

fn p1_max_joltage(battery: &Battery, k_picks: u8) -> u64 {
	let mut max_joltage: Vec<Vec<u64>> = vec![vec![0; (k_picks + 1) as usize]];
	battery.joltages.iter().enumerate().for_each(|(n, &cell)| {
		max_joltage.push(vec![0]);
		for k in 1..=k_picks as usize {
			let next = max(max_joltage[n][k], 10 * max_joltage[n][k - 1] + cell as u64);
			max_joltage[n + 1].push(next);
		}
	});
	// println!("{:?}", max_joltage);
	max_joltage
		.last()
		.unwrap_or(&vec![0])
		.last()
		.unwrap_or(&0)
		.to_owned() as u64
}

pub fn p1(batteries: &[Battery]) -> u64 {
	batteries.iter().map(|x| p1_max_joltage(x, 2)).sum()
}

pub fn p2(batteries: &[Battery]) -> u64 {
	batteries.iter().map(|x| p1_max_joltage(x, 12)).sum()
}
