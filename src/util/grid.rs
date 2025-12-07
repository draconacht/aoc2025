use std::fmt::Display;

#[derive(Clone)]
pub struct Grid<X>(pub Vec<Vec<X>>);

impl<X> Grid<X> {
	pub fn safe_get(&mut self, i: usize, j: usize) -> Option<&mut X> {
		self.0.get_mut(i)?.get_mut(j)
	}
}

impl<X: Display> Display for Grid<X> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let towrite = self
			.0
			.iter()
			.map(|s| s.iter().map(|cell| format!("{}", cell)).collect::<String>() + "\n")
			.collect::<String>();
		f.write_str(towrite.as_str())
	}
}
