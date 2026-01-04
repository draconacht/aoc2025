use itertools::Itertools;
use ndarray::{Array2, Axis};
use num_traits::{Num, PrimInt};

use crate::qtui::render::{Cell, Color, ToColor};

pub struct Grid {}

impl super::render::Element<Vec<Vec<Color>>> for Grid {
	fn render(&self, inp: Vec<Vec<Color>>) -> Vec<Vec<Cell>> {
		inp.into_iter()
			.map(|x| x.into_iter().map(|y| Cell { color: y, text: ' ' }).collect_vec())
			.collect_vec()
	}
}

impl<X: ToColor> super::render::Element<&Array2<X>> for Grid {
	fn render(&self, inp: &Array2<X>) -> Vec<Vec<Cell>> {
		inp.map(|y| Cell {
			color: y.to_color(),
			text: ' ',
		})
		.axis_iter(Axis(0))
		.map(|x| x.to_vec())
		.collect_vec()
	}
}
