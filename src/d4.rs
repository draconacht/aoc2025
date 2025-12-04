use std::{fs::read_to_string, ops::Add, path::Path, str::FromStr};

use crate::util::errors::MyError;

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
	Wall(u8), // wall is either picked or unpicked
	Hole,
}

fn get2mut<T>(v: &mut Vec<Vec<T>>, i: usize, j: usize) -> Option<&mut T> {
	v.get_mut(i)?.get_mut(j)
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

impl Cell {
	fn bump_if_wall(&mut self) {
		if let Self::Wall(x) = self {
			*self = Self::Wall(*x + 1)
		}
	}
	fn debump_if_wall(&mut self) {
		if let Self::Wall(x) = self {
			*self = Self::Wall(*x - 1)
		}
	}
	fn pick(&mut self) {
		*self = Self::Hole
	}
}

pub fn load(flname: impl AsRef<Path>) -> Result<Vec<Vec<Cell>>, MyError> {
	read_to_string(flname)?
		.lines()
		.map(|line| {
			line.chars()
				.map(|ch| Cell::from_str(ch.to_string().as_str()))
				.collect::<Result<Vec<Cell>, _>>()
		})
		.collect()
}

pub fn p1(grid: &[&[Cell]]) -> u32 {
	let mut g: Vec<Vec<Cell>> = grid.clone().iter().map(|x| x.to_vec()).collect::<Vec<Vec<Cell>>>();
	let dirs: Vec<(i8, i8)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
	// bump walls around walls
	for (i, row) in grid.iter().enumerate() {
		for (j, cell) in row.iter().enumerate() {
			if let Cell::Wall(_) = cell {
				for (x, y) in &dirs {
					get2mut(&mut g, (i as i32 + *x as i32) as usize, (j as i32 + *y as i32) as usize)
						.map(|x| x.bump_if_wall());
				}
			}
		}
	}

	let m: Vec<&[Cell]> = g.iter().map(|x| x.as_slice()).collect();
	println!("final");
	m.iter()
		.flat_map(|x| x.iter())
		.filter(|x| {
			if let Cell::Wall(neighbours) = x
				&& *neighbours < 4
			{
				true
			} else {
				false
			}
		})
		.count() as u32
}

pub fn p2(grid: &[&[Cell]]) -> u32 {
	let mut g: Vec<Vec<Cell>> = grid.clone().iter().map(|x| x.to_vec()).collect::<Vec<Vec<Cell>>>();
	let dirs: Vec<(i8, i8)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
	// bump walls around walls
	let mut acc = 0;
	let mut curr = 1;

	for (i, row) in grid.iter().enumerate() {
		for (j, cell) in row.iter().enumerate() {
			if let Cell::Wall(_) = cell {
				for (x, y) in &dirs {
					get2mut(&mut g, (i as i32 + *x as i32) as usize, (j as i32 + *y as i32) as usize)
						.map(|x| x.bump_if_wall());
				}
			}
		}
	}

	while curr > 0 {
		curr = 0;
		for (i, row) in grid.iter().enumerate() {
			for (j, _) in row.iter().enumerate() {
				if let Cell::Wall(neighbours) = g[i][j]
					&& neighbours < 4
				{
					for (x, y) in &dirs {
						get2mut(&mut g, (i as i32 + *x as i32) as usize, (j as i32 + *y as i32) as usize)
							.map(|x| x.debump_if_wall());
					}
					curr += 1;
					get2mut(&mut g, i, j).map(|x| x.pick());
				}
			}
		}
		println!("curr: {}", curr);
		acc += curr
	}
	acc
}
