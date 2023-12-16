#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right
}

impl Direction {
	pub fn step(&self) -> (isize, isize) {
		match self {
			Direction::Up => (0, -1),
			Direction::Down => (0, 1),
			Direction::Left => (-1, 0),
			Direction::Right => (1, 0),
		}
	}
}

pub enum Tile {
	Space,
	Mirror, // '/'
	BackMirror, // '\'
	HorizontalSplitter, // -
	VerticalSplitter, // |
}

pub type Grid = Vec<Vec<Tile>>;

impl Tile {
	pub fn apply(&self, direction: &Direction) -> Action {
		match (self, direction) {
			(Tile::Mirror, _) => Action::NewDirection(Tile::apply_mirror(direction)),
			(Tile::BackMirror, _) => Action::NewDirection(Tile::apply_back_mirror(direction)),
			(Tile::HorizontalSplitter, Direction::Up) => Action::HorizontalSplit,
			(Tile::HorizontalSplitter, Direction::Down) => Action::HorizontalSplit,
			(Tile::VerticalSplitter, Direction::Left) => Action::VerticalSplit,
			(Tile::VerticalSplitter, Direction::Right) => Action::VerticalSplit,
			_ => Action::None
		}
	}

	//   /
	fn apply_mirror(direction: &Direction) -> Direction {
		match direction {
			Direction::Up => Direction::Right,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Down,
			Direction::Right => Direction::Up,
		}
	}

	//   \
	fn apply_back_mirror(direction: &Direction) -> Direction {
		match direction {
			Direction::Up => Direction::Left,
			Direction::Down => Direction::Right,
			Direction::Left => Direction::Up,
			Direction::Right => Direction::Down,
		}
	}
}

pub enum Action {
	None,
	NewDirection(Direction),
	VerticalSplit,
	HorizontalSplit
}

#[derive(Debug, Clone)]
pub struct Beam {
	pub direction: Direction,
	pub x: isize,
	pub y: isize,
}

impl Beam {
	pub fn new(direction: Direction, x: isize, y: isize) -> Self {
		Self { direction, x, y }
	}

	pub fn start() -> Self {
		Beam::new(Direction::Right, 0, 0)
	}

	pub fn step(&mut self) {
		let (dx, dy) = self.direction.step();
		self.x += dx;
		self.y += dy;
	}
}

pub struct Map {
	grid: Grid
}

impl Map {
	pub fn new(grid: Grid) -> Self {
		Self { grid }
	}

	pub fn get(&self, x: isize, y: isize) -> Option<&Tile> {
		if x < 0 || y < 0 {
			None
		} else {
			self.grid.get(y as usize).and_then(|line| line.get(x as usize))
		}
	}

	pub fn in_bounds(&self, x: isize, y: isize) -> bool {
		self.get(x, y).is_some()
	}

	pub fn width(&self) -> usize {
		self.grid[0].len()
	}
	
	pub fn height(&self) -> usize {
		self.grid.len()
	}
}