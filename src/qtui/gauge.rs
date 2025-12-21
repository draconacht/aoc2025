use itertools::Itertools;
use num_traits::{Num, PrimInt};

use crate::qtui::render::{Cell, Color};

pub struct Gauge {
	pub max_val: u64, // max val on either side of gauge
	pub size: u64,    // one element in this is zero, so it is recommended to keep this odd
	pub offset: u64,
}

impl<X: PrimInt> super::render::Element<X> for Gauge {
	fn render(&self, inp: X) -> Vec<Vec<Cell>> {
		let out = (0..self.size)
			.map(|p| {
				let p_moved = p + self.offset;
				let p_actual = (p_moved * 2 * self.max_val / (self.size - 1)) as i64 - (self.max_val) as i64;
				let col = if p_actual <= 0 && inp.to_i64().unwrap() <= p_actual
					|| p_actual >= 0 && inp.to_i64().unwrap() >= p_actual
				{
					Color::Primary
				} else {
					Color::Secondary
				};
				Cell { color: col, text: ' ' }
			})
			.collect_vec();
		vec![out]
	}
}
