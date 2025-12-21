use crate::qtui::{
	gauge::Gauge,
	render::{Renderer, default_colorizer},
};

use super::qtui;
use anyhow::{Result, bail};
use itertools::Itertools;
use std::{fs::read_to_string, path::Path, str::FromStr, time::Duration};

#[derive(Debug, PartialEq)]
pub struct Rotation(i16);

impl FromStr for Rotation {
	type Err = anyhow::Error;
	fn from_str(s: &str) -> Result<Self> {
		match s.chars().nth(0) {
			Some('L') => Ok(Rotation(-s[1..].parse::<i16>()?)),
			Some('R') => Ok(Rotation(s[1..].parse::<i16>()?)),
			Some(_) => bail!("invalid char"),
			None => bail!("stop"),
		}
	}
}

pub fn load(flname: impl AsRef<Path>) -> Result<Vec<Rotation>> {
	let fl = read_to_string(flname)?;
	fl.lines().map(Rotation::from_str).try_collect()
}

pub fn p1(dirs: &[Rotation]) -> u16 {
	let (mut angle, mut zeros) = (50, 0);
	for dir in dirs {
		angle = (angle + dir.0).rem_euclid(100);
		if angle == 0 {
			zeros += 1;
		}
	}
	zeros
}

pub fn p2(dirs: &[Rotation]) -> u16 {
	let (mut angle, mut zeros) = (50, 0);
	for dir in dirs {
		let (full_rotations, remainder) = (dir.0 / 100, dir.0 % 100);
		zeros += full_rotations.unsigned_abs();
		if angle + remainder <= 0 && angle != 0 || angle + remainder >= 100 {
			zeros += 1;
		}
		angle = (angle + dir.0).rem_euclid(100);
	}
	zeros
}

pub fn vis(dirs: &[Rotation]) -> u16 {
	let g = Gauge {
		max_val: 50,
		size: 100,
		offset: 0,
	};
	let r = Renderer {
		sleep: Duration::from_millis(500),
		clrzr: default_colorizer,
	};
	let (mut angle, mut zeros) = (50, 0);
	for dir in dirs {
		let a = (angle + dir.0).rem_euclid(100);
		let d_angle = 50 - 2 * ((a + 25).rem_euclid(100) - 50).abs();
		r.render(&g, d_angle, format!("{} + {} = {}", angle, dir.0, a).as_str());
		angle = a;
		if angle == 0 {
			zeros += 1;
		}
	}
	zeros
}
