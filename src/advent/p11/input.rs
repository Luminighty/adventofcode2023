use super::{error::AdventError, galaxy::{Tile, Image}};


pub fn get_input() -> Result<Image, AdventError> {
	let content = std::fs::read_to_string("inp/11.txt")
		.map_err(|_| AdventError::FileReadError)?;

	let image = content
		.lines()
		.map(|line|
			line.chars()
				.map(parse_tile)
				.collect()
		)
		.collect::<Result<Vec<Vec<Tile>>, AdventError>>()?;
	Ok(Image::new(image))
}

fn parse_tile(c: char) -> Result<Tile, AdventError> {
	match c {
		'.' => Ok(Tile::Space),
		'#' => Ok(Tile::Galaxy),
		_ => Err(AdventError::UnknownTile(c))
	}
}