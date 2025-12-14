use std::{collections::HashMap, fs::read_to_string, ops::Div, path::Path, str::FromStr};

use crate::util::errors::MyError;
use itertools::Itertools;

#[derive(Debug, Default, Clone)]
pub struct Nonamino {
	cells: [[bool; 3]; 3],
}

impl Nonamino {
	pub fn filled_cells(&self) -> u8 {
		self.cells.iter().flatten().filter(|x| **x).count() as u8
	}
	pub fn efficiency(&self) -> f64 {
		let (trues, falses): (Vec<bool>, Vec<bool>) = self.cells.iter().flatten().partition(|x| **x);
		trues.len() as f64 / (trues.len() + falses.len()) as f64
	}
}

#[derive(Debug)]
pub struct Puzzle {
	bounding_box: (u8, u8),
	budgets: Vec<u64>,
}

impl Puzzle {
	pub fn required_efficiency(&self, nonaminos: &[Nonamino]) -> f64 {
		let all_cells = self.bounding_box.0 as u64 * self.bounding_box.1 as u64;
		let i_budgets = self.budgets.iter().enumerate();
		let filled_cells = i_budgets
			.map(|(i, n)| nonaminos[i].filled_cells() as u64 * *n)
			.sum::<u64>();
		filled_cells as f64 / all_cells as f64
	}
	pub fn naive_efficiency(&self, nonaminos: &[Nonamino]) -> f64 {
		let i_budgets = self.budgets.iter().enumerate();
		i_budgets
			.map(|(i, n)| nonaminos[i].efficiency() * *n as f64)
			.sum::<f64>()
			.div(self.budgets.iter().sum::<u64>() as f64)
	}
}

impl FromStr for Nonamino {
	type Err = MyError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut brick = Self::default();
		for (i, line) in s.lines().enumerate() {
			for (j, ch) in line.chars().enumerate() {
				if ch == '#' {
					brick.cells[i][j] = true;
				}
			}
		}
		Ok(brick)
	}
}

impl FromStr for Puzzle {
	type Err = MyError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (bx, bdg) = s.split_once(": ").ok_or("no label")?;
		let (x, y) = bx.split_once("x").ok_or("no X")?;
		let bounding_box = (x.parse::<u8>()?, y.parse::<u8>()?);
		let budgets = bdg.split(" ").map(u64::from_str).try_collect()?;
		Ok(Self { bounding_box, budgets })
	}
}

pub fn load(flname: impl AsRef<Path>) -> Result<(Vec<Nonamino>, Vec<Puzzle>), MyError> {
	let txt = read_to_string(flname)?;
	let pieces = txt.split("\n\n");
	let puzzles_chunk = pieces.clone().last().unwrap();
	let puzzles = puzzles_chunk.lines().map(Puzzle::from_str).try_collect()?;
	let bricks = pieces
		.map(|x| x.split_once("\n").unwrap().1)
		.map(Nonamino::from_str)
		.try_collect()?;
	Ok((bricks, puzzles))
}
