use std::{
	collections::HashSet,
	fs::read_to_string,
	path::Path,
	str::FromStr,
	thread::sleep,
	time::{Duration, Instant},
};

use itertools::Itertools;
use ndarray::Array2;

use crate::{
	qtui::{
		self,
		render::{Color, Renderer, ToColor, default_colorizer},
	},
	util::errors::MyError,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
	Wall(u8), // wall is either picked or unpicked
	Hole(u8),
}

impl FromStr for Cell {
	type Err = MyError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"@" => Ok(Self::Wall(0)),
			"." => Ok(Self::Hole(0)),
			_ => Err("invalid string passed")?,
		}
	}
}

fn offset(start: usize, offset: i8) -> usize {
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
		*self = Self::Hole(0)
	}
}

impl ToColor for Cell {
	fn to_color(&self) -> Color {
		match self {
			Cell::Wall(x) => Color::LinX(
				colored::CustomColor::new(60, 70, 140),
				colored::CustomColor::new(200, 210, 140),
				*x as f32 / 10.0,
			),
			Cell::Hole(x) => Color::LinX(
				colored::CustomColor::new(40, 40, 180),
				colored::CustomColor::new(60, 70, 140),
				*x as f32 / 10.0,
			),
		}
	}
}

pub fn load(flname: impl AsRef<Path>) -> Result<Array2<Cell>, MyError> {
	let dt: Vec<Cell> = read_to_string(flname)?
		.lines()
		.flat_map(|line| {
			line.chars()
				.map(|ch| Cell::from_str(ch.to_string().as_str()))
				.collect_vec()
		})
		.try_collect()?;
	Ok(Array2::from_shape_vec((dt.len().isqrt(), dt.len().isqrt()), dt)?)
}

pub fn p1(grid: &mut Array2<Cell>) -> u32 {
	let idx = grid.clone();
	let dirs: Vec<(i8, i8)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

	// bump walls around walls
	for ((i, j), _) in idx.indexed_iter() {
		if let Cell::Hole(_) = grid[(i, j)] {
			continue;
		}
		for (x, y) in &dirs {
			if let Some(c) = grid.get_mut((offset(i, *x), offset(j, *y))) {
				c.bump_if_wall()
			}
		}
	}

	grid.iter().filter(|x| matches!(x, Cell::Wall(..4))).count() as u32
}

pub fn p2(grid: &mut Array2<Cell>) -> u32 {
	let idx = grid.clone();
	let dirs: Vec<(i8, i8)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

	// bump walls around walls
	for ((i, j), _) in idx.indexed_iter() {
		if let Cell::Hole(_) = grid[(i, j)] {
			continue;
		}
		for (x, y) in &dirs {
			let (ix, jx) = (offset(i, *x), offset(j, *y));
			grid.get_mut((ix, jx)).map(|c| c.bump_if_wall());
		}
	}

	let mut acc = 0;
	let mut just_picked = 1;

	// pick vulnerable walls and debump their neighbours
	let mut iter = 0;
	while just_picked > 0 {
		iter += 1;
		just_picked = 0;

		for ((i, j), _) in idx.indexed_iter() {
			if !matches!(grid[(i, j)], Cell::Wall(..4)) {
				continue;
			}

			grid[(i, j)] = Cell::Hole(iter);
			just_picked += 1;

			for (x, y) in &dirs {
				let (ix, jx) = (offset(i, *x), offset(j, *y));
				grid.get_mut((ix, jx)).map(|c| c.debump_if_wall());
			}
		}
		acc += just_picked
	}

	acc
}

pub fn p2_flood(grid: &mut Array2<Cell>) -> u32 {
	let idx = grid.clone();
	let dirs: Vec<(i8, i8)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

	// bump walls around walls
	for ((i, j), _) in idx.indexed_iter() {
		if let Cell::Hole(_) = grid[(i, j)] {
			continue;
		}
		for (x, y) in &dirs {
			let (ix, jx) = (offset(i, *x), offset(j, *y));
			grid.get_mut((ix, jx)).map(|c| c.bump_if_wall());
		}
	}

	let mut acc = 0;
	let mut buffer = vec![false; 19044];
	for ((i, j), _) in idx.indexed_iter() {
		let mut keys = vec![];
		buffer[i * 138 + j] = true;
		keys.push((i, j));
		while let Some((a, b)) = keys.pop() {
			if !buffer[a * 138 + b] {
				continue;
			}
			buffer[a * 138 + b] = false;
			if !matches!(grid[(a, b)], Cell::Wall(..4)) {
				continue;
			}

			for (x, y) in &dirs {
				let (m, n) = (offset(a, *x), offset(b, *y));
				if let Some(p) = grid.get_mut((m, n)) {
					p.debump_if_wall();
					if !buffer[m * 138 + n] {
						buffer[m * 138 + n] = true;
						keys.push((m, n))
					}
				}
			}
			acc += 1;

			grid[(a, b)] = Cell::Hole((acc / 800) as u8);
		}
	}
	acc
}

pub fn vis_linear(mut grid: Array2<Cell>) -> u32 {
	let idx = grid.clone();
	let g = qtui::grid::Grid {};
	let mut r = Renderer::new(Duration::from_millis(20), default_colorizer);
	let dirs: Vec<(i8, i8)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

	// bump walls around walls
	for ((i, j), _) in idx.indexed_iter() {
		if let Cell::Hole(_) = grid[(i, j)] {
			continue;
		}
		for (x, y) in &dirs {
			let (ix, jx) = (offset(i, *x), offset(j, *y));
			grid.get_mut((ix, jx)).map(|c| c.bump_if_wall());
		}
	}

	let mut acc = 0;
	let mut just_picked = 1;

	// pick vulnerable walls and debump their neighbours
	let mut iter = 0;
	while just_picked > 0 {
		iter += 1;
		just_picked = 0;

		for ((i, j), _) in idx.indexed_iter() {
			r.render(&g, &grid, format!("generation {iter}").as_str());
			if !matches!(grid[(i, j)], Cell::Wall(..4)) {
				continue;
			}
			sleep(Duration::from_nanos(100_000));

			grid[(i, j)] = Cell::Hole(iter);
			just_picked += 1;

			for (x, y) in &dirs {
				let (ix, jx) = (offset(i, *x), offset(j, *y));
				grid.get_mut((ix, jx)).map(|c| c.debump_if_wall());
			}
		}
		acc += just_picked
	}

	acc
}

pub fn vis_eager_flood(mut grid: Array2<Cell>) -> u32 {
	let g = qtui::grid::Grid {};
	let mut r = Renderer::new(Duration::from_millis(10), default_colorizer);
	let idx = grid.clone();
	let dirs: Vec<(i8, i8)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
	let mut acc = 0;

	for ((i, j), _) in idx.indexed_iter() {
		if let Cell::Hole(_) = grid[(i, j)] {
			continue;
		}
		for (x, y) in &dirs {
			let (ix, jx) = (offset(i, *x), offset(j, *y));
			grid.get_mut((ix, jx)).map(|c| c.bump_if_wall());
		}
	}

	for ((i, j), _) in idx.indexed_iter() {
		let mut buffer = vec![(i, j)];
		while let Some((a, b)) = buffer.pop() {
			if !matches!(grid[(a, b)], Cell::Wall(..4)) {
				continue;
			}
			sleep(Duration::from_nanos(100_000));

			for (x, y) in &dirs {
				let (m, n) = (offset(a, *x), offset(b, *y));
				if let Some(p) = grid.get_mut((m, n)) {
					p.debump_if_wall();
					if !buffer.contains(&(m, n)) {
						buffer.push((m, n));
					}
				}
			}
			acc += 1;

			grid[(a, b)] = Cell::Hole((acc / 800) as u8);
			r.render(&g, &grid, "");
		}
	}
	acc
}

pub fn vis_lazy_flood(mut grid: Array2<Cell>) -> u32 {
	let g = qtui::grid::Grid {};
	let mut r = Renderer::new(Duration::from_millis(15), default_colorizer);
	let idx = grid.clone();
	let dirs: Vec<(i8, i8)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
	let mut acc = 0;

	for ((i, j), _) in idx.indexed_iter() {
		if let Cell::Hole(_) = grid[(i, j)] {
			continue;
		}
		for (x, y) in &dirs {
			let (ix, jx) = (offset(i, *x), offset(j, *y));
			grid.get_mut((ix, jx)).map(|c| c.bump_if_wall());
		}
	}

	let mut buffer = vec![];
	for ((i, j), _) in idx.indexed_iter() {
		buffer.push((i, j));
	}
	while !buffer.is_empty() {
		let mut next_buffer = vec![];
		while let Some((a, b)) = buffer.pop() {
			if !matches!(grid[(a, b)], Cell::Wall(..4)) {
				continue;
			}
			sleep(Duration::from_nanos(200_000));

			for (x, y) in &dirs {
				let (m, n) = (offset(a, *x), offset(b, *y));
				if let Some(p) = grid.get_mut((m, n)) {
					p.debump_if_wall();
					if !next_buffer.contains(&(m, n)) {
						next_buffer.push((m, n));
					}
				}
			}
			acc += 1;

			grid[(a, b)] = Cell::Hole((acc / 800) as u8);
			r.render(&g, &grid, "");
		}
		buffer = next_buffer;
	}
	acc
}
