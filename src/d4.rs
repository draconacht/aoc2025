use std::{fs::read_to_string, ops::Add, path::Path, str::FromStr};

use crate::util::{errors::MyError, grid::Grid};

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
	Wall(u8), // wall is either picked or unpicked
	Hole,
}

impl FromStr for Cell {
	type Err = MyError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"@" => Ok(Self::Wall(0)),
			"." => Ok(Self::Hole),
			_ => Err("invalid string passed")?,
		}
	}
}

fn offset2d(start: usize, offset: i8) -> usize {
	(start as isize + offset as isize) as usize
}

impl Cell {
	fn bump_if_wall(&mut self) {
		if let Self::Wall(x) = self {
			*self = Self::Wall(*x + 1)
		}
	}
	fn debump_if_wall(&mut self) {
		if let Self::Wall(x) = self {
			*self = Self::Wall(x.saturating_sub(1))
		}
	}
	fn pick(&mut self) {
		*self = Self::Hole
	}
}

pub fn load(flname: impl AsRef<Path>) -> Result<Grid<Cell>, MyError> {
	let outvec = read_to_string(flname)?
		.lines()
		.map(|line| {
			line.chars()
				.map(|ch| Cell::from_str(ch.to_string().as_str()))
				.collect::<Result<Vec<Cell>, _>>()
		})
		.collect::<Result<Vec<_>, _>>()?;
	Ok(Grid(outvec))
}

pub fn p1(grid: &mut Grid<Cell>) -> u32 {
	let idx = grid.clone();
	let dirs: Vec<(i8, i8)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
	// bump walls around walls
	for (i, row) in idx.0.iter().enumerate() {
		for (j, cell) in row.iter().enumerate() {
			if let Cell::Hole = cell {
				continue;
			}
			for (x, y) in &dirs {
				grid.safe_get(offset2d(i, *x), offset2d(j, *y))
					.map(|x| x.bump_if_wall());
			}
		}
	}
	grid.0.iter().flatten().filter(|x| matches!(x, Cell::Wall(..4))).count() as u32
}

pub fn p2(grid: &mut Grid<Cell>) -> u32 {
	let idx = grid.clone();
	let dirs: Vec<(i8, i8)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
	let mut acc = 0;
	let mut curr = 1;

	// bump walls around walls
	for (i, row) in idx.0.iter().enumerate() {
		for (j, cell) in row.iter().enumerate() {
			if let Cell::Hole = cell {
				continue;
			}
			for (x, y) in &dirs {
				grid.safe_get(offset2d(i, *x), offset2d(j, *y))
					.map(|x| x.bump_if_wall());
			}
		}
	}

	while curr > 0 {
		curr = 0;
		for (i, row) in idx.0.iter().enumerate() {
			for (j, _) in row.iter().enumerate() {
				if let Cell::Wall(neighbours) = grid.0[i][j]
					&& neighbours < 4
				{
					for (x, y) in &dirs {
						grid.safe_get(offset2d(i, *x), offset2d(j, *y))
							.map(|x| x.debump_if_wall());
					}
					curr += 1;
					grid.0[i][j].pick();
				}
			}
		}
		println!("curr: {}", curr);
		acc += curr
	}
	acc
}
