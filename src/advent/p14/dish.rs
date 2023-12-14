#[derive(PartialEq, Eq)]
pub enum Tile {
	RoundRock,
	CubeRock,
	Space,
}

pub type Grid = Vec<Vec<Tile>>;

pub type Pos = (usize, usize);

pub struct Dish {
	grid: Grid,
}

impl Dish {
	pub fn new(grid: Grid) -> Self {
		Self { grid }
	}

	fn group(&self, tile: Tile) -> Vec<Pos> {
		let mut v = Vec::new();
		for y in (0..self.grid.len()).rev() {
			for x in 0..self.grid[y].len() {
				if self.grid[y][x] == tile {
					v.push((x, y));
				}
			}
		}
		v
	}

	pub fn height(&self) -> usize {
		self.grid.len()
	}

	pub fn width(&self) -> usize {
		self.grid[0].len()
	}

	pub fn round_rocks(&self) -> Vec<Pos> {
		self.group(Tile::RoundRock)
	}
	
	pub fn cube_rocks(&self) -> Vec<Pos> {
		self.group(Tile::CubeRock)
	}

}