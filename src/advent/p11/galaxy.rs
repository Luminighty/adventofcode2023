use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
	Space,
	Galaxy
}

impl Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Tile::Galaxy => "#",
			Tile::Space => ".",
		})
	}
}

pub type Expansion = (Vec<isize>, Vec<isize>);
pub struct Image {
	image: Vec<Vec<Tile>>
}

impl Display for Image {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in &self.image {
			for tile in row {
				write!(f, "{}", tile)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}


impl Image {
	pub fn new(image: Vec<Vec<Tile>>) -> Self {
		Self { image }
	}

	pub fn expand(&mut self) {
		for x in (0..self.width()).rev() {
			if self.is_column_empty(x) {
				self.expand_column(x)
			}
		}
		for y in (0..self.height()).rev() {
			if self.is_row_empty(y) {
				self.expand_row(y)
			}
		}
	}

	pub fn expand_meta(&self) -> Expansion {
		let mut columns = Vec::new();
		let mut rows = Vec::new();

		for x in (0..self.width()).rev() {
			if self.is_column_empty(x) {
				columns.push(x as isize);
			}
		}
		for y in (0..self.height()).rev() {
			if self.is_row_empty(y) {
				rows.push(y as isize);
			}
		}
		(columns, rows)
	}

	pub fn galaxies(&self) -> Vec<(isize, isize)> {
		let mut galaxies = Vec::new();
		for (y, row) in self.image.iter().enumerate() {
			for (x, tile) in row.iter().enumerate() {
				if tile == &Tile::Galaxy {
					galaxies.push((x as isize, y as isize))
				}
			}
		}
		galaxies
	}

	fn expand_column(&mut self, x: usize) {
		for row in &mut self.image {
			row.insert(x, Tile::Space);
		}
	}
	
	fn expand_row(&mut self, y: usize) {
		self.image.insert(y, vec![Tile::Space; self.width()])
	}

	pub fn width(&self) -> usize {
		self.image[0].len()
	}
	
	pub fn height(&self) -> usize {
		self.image.len()
	}

	pub fn is_row_empty(&self, y: usize) -> bool {
		for tile in &self.image[y] {
			if tile == &Tile::Galaxy {
				return false;
			}
		}
		return true;
	}
	
	pub fn is_column_empty(&self, x: usize) -> bool {
		for row in &self.image {
			if row[x] == Tile::Galaxy {
				return false;
			}
		}
		return true;
	}
}