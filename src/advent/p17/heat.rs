pub type Item = u32;
pub type Grid = Vec<Vec<Item>>;

#[derive(Debug)]
pub struct Map {
	grid: Grid,
}

impl Map {
	pub fn new(grid: Grid) -> Self {
		Self { grid }
	}

	pub fn get(&self, x: isize, y: isize) -> Option<&Item> {
		if x < 0 || y < 0 {
			None
		} else {
			self.grid.get(y as usize).and_then(|l| l.get(x as usize))
		}
	}

	pub fn height(&self) -> usize {
		self.grid.len()
	}

	pub fn width(&self, y: usize) -> usize {
		self.grid.get(y).map(|l| l.len()).unwrap_or(0)
	}
}