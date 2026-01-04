use std::{fmt::Display, fs::read_to_string, path::Path, thread::sleep, time::Duration};

use colored::Colorize;

use crate::util::errors::MyError;

#[derive(Debug, Clone)]
pub enum Cell {
	Star,
	Splitter,
	Hole(u64),
}

impl TryFrom<char> for Cell {
	type Error = MyError;
	fn try_from(s: char) -> Result<Self, MyError> {
		match s {
			'^' => Ok(Self::Splitter),
			'S' => Ok(Self::Star),
			'.' => Ok(Self::Hole(0)),
			_ => Err("in(valid)cell")?,
		}
	}
}

fn gradient(start: (u8, u8, u8), end: (u8, u8, u8), offset: f64) -> (u8, u8, u8) {
	let r = (start.0 as f64) + (offset * (end.0 as f64 - start.0 as f64));
	let g = (start.1 as f64) + (offset * (end.1 as f64 - start.1 as f64));
	let b = (start.2 as f64) + (offset * (end.2 as f64 - start.2 as f64));
	(r as u8, g as u8, b as u8)
}

impl Display for Cell {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let x = match self {
			Self::Splitter => format!("{}", " ".on_black()),
			Self::Star => format!("{}", "*".on_yellow()),
			Self::Hole(0) => " ".to_string(),
			Self::Hole(x) => {
				if x % 2 == 0 {
					let (r, g, b) = gradient((40, 40, 40), (255, 40, 40), (*x as f64).log10() / 17.0);
					format!("{}", " ".on_truecolor(r, g, b))
				} else {
					let (r, g, b) = gradient((40, 40, 40), (255, 40, 40), 0.0);
					format!("{}", " ".on_truecolor(r, g, b))
				}
			}
		};
		f.write_str(x.as_str())
	}
}

impl Cell {
	fn bump_if_hole(&mut self, by: u64) {
		if let Self::Hole(x) = self {
			*self = Self::Hole(*x + by)
		}
	}
}

pub fn load(flname: impl AsRef<Path>) -> Result<(usize, Vec<Vec<Cell>>), MyError> {
	let file = read_to_string(flname)?;
	let lines = file
		.lines()
		.map(|x| x.chars().map(Cell::try_from).collect::<Result<Vec<_>, _>>())
		.collect::<Result<Vec<_>, _>>()?;
	let pos_star = file.lines().next().ok_or("empty?")?.find("S").ok_or("no stars?")?;
	Ok((pos_star, lines))
}

pub fn p1(start: usize, rows: &mut [Vec<Cell>]) -> usize {
	rows[1][start].bump_if_hole(1);
	let mut splits = 0;
	for i in 2..rows.len() {
		for j in 0..rows[0].len() {
			if !matches!(rows[i - 1][j], Cell::Hole(x) if x > 0) {
				continue;
			}
			match rows[i][j] {
				Cell::Hole(_) => rows[i][j].bump_if_hole(1),
				Cell::Splitter => {
					splits += 1;
					rows[i][j - 1].bump_if_hole(1);
					rows[i][j + 1].bump_if_hole(1);
				}
				_ => continue,
			}
		}
	}
	splits
}

pub fn p2(start: usize, rows: &mut [Vec<Cell>]) -> u64 {
	rows[1][start].bump_if_hole(1);
	for i in 2..rows.len() {
		for j in 0..rows[0].len() {
			let bumps = match rows[i - 1][j] {
				Cell::Hole(x) if x > 0 => x,
				_ => continue,
			};

			match rows[i][j] {
				Cell::Hole(_) => rows[i][j].bump_if_hole(bumps),
				Cell::Splitter => {
					rows[i][j - 1].bump_if_hole(bumps);
					rows[i][j + 1].bump_if_hole(bumps);
				}
				_ => continue,
			}
		}
		// print!("\x1B[2J\x1B[1;1H");
		// println!("{}", Grid(rows.to_vec()));
		sleep(Duration::from_millis(90));
	}
	let last_row = rows.last().unwrap().iter();
	last_row.map(|x| if let Cell::Hole(n) = x { *n } else { 0 }).sum()
}
