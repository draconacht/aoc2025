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
	let fdata = read_to_string(flname)?;
	fdata.trim().lines().map(Battery::from_str).collect()
}

fn dp_max_joltage(battery: &Battery, k_picks: u8) -> u64 {
	let mut max_joltage: Vec<Vec<u64>> = vec![vec![0; (k_picks + 1) as usize]];
	battery.joltages.iter().enumerate().for_each(|(n, &cell)| {
		max_joltage.push(vec![0]);
		for k in 1..=k_picks as usize {
			let next = max(max_joltage[n][k], 10 * max_joltage[n][k - 1] + cell as u64);
			max_joltage[n + 1].push(next);
		}
	});
	// println!("{:?}", max_joltage);
	max_joltage[battery.joltages.len()][k_picks as usize]
}

fn greedy_max_joltage(battery: &Battery, k_picks: u8) -> u64 {
	let mut max_ahead: Vec<(usize, u8)> = vec![(0, 0); battery.joltages.len() + 1];
	let bats = battery.joltages.iter().enumerate().rev();
	bats.for_each(|(i, &joltage)| {
		let (max_pos, max_val) = max_ahead[i + 1];
		if max_val > joltage {
			max_ahead[i] = (max_pos, max_val)
		} else {
			max_ahead[i] = (i, joltage)
		}
	});
	let mut out = 0u64;
	let mut k = k_picks as usize;

	for (i, &v) in battery.joltages.iter().enumerate() {
		if k == 0 {
			break;
		}
		if v >= max_ahead[i + 1].1 || i + k >= battery.joltages.len() {
			(out, k) = (out * 10 + v as u64, k - 1)
		}
	}
	println!("{}", out);
	out
}

pub fn p1_slower(batteries: &[Battery]) -> u64 {
	batteries.iter().map(|x| dp_max_joltage(x, 2)).sum()
}

pub fn p1(batteries: &[Battery]) -> u64 {
	batteries.iter().map(|x| greedy_max_joltage(x, 2)).sum()
}

pub fn p2_slower(batteries: &[Battery]) -> u64 {
	batteries.iter().map(|x| dp_max_joltage(x, 12)).sum()
}
