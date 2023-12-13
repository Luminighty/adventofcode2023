use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Tile {
	Ash,
	Rock
}

impl Debug for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Tile::Ash => '.',
			Tile::Rock => '#',
		})
	}
}

pub type Grid = Vec<Vec<Tile>>;

pub struct Map {
	map: Grid
}

impl Debug for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for line in &self.map {
			for tile in line {
				write!(f, "{:?}", tile)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl Map {
	pub fn new(map: Grid) -> Self {
		Self { map }
	}

	pub fn height(&self) -> usize {
		self.map.len()
	}

	pub fn width(&self, row: usize) -> usize {
		self.map[row].len()
	}

	pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
		self.map.get(y).and_then(|row| row.get(x))
	}
}
