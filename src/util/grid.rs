#[derive(Clone)]
pub struct Grid<X>(pub Vec<Vec<X>>);

impl<X> Grid<X> {
	pub fn safe_get(&mut self, i: usize, j: usize) -> Option<&mut X> {
		self.0.get_mut(i)?.get_mut(j)
	}
}
