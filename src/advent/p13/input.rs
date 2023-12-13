use super::{error::AdventError, map::{Map, Tile, Grid}};

pub fn get_input() -> Result<Vec<Map>, AdventError> {
	let content = std::fs::read_to_string("inp/13.txt")
		.map_err(|_| AdventError::FileReadError)?;

	content.split("\n\n")
		.map(parse_map)
		.collect()
}

fn parse_map(map: &str) -> Result<Map, AdventError> {
	let map: Grid =	map
		.lines()
		.map(|line|
			line.chars()
				.map(parse_tile)
				.collect::<Result<Vec<Tile>, AdventError>>()
		)
		.collect::<Result<Grid, AdventError>>()?;
	Ok(Map::new(map))
}

fn parse_tile(tile: char) -> Result<Tile, AdventError> {
	match tile {
		'.' => Ok(Tile::Ash),
		'#' => Ok(Tile::Rock),
		_ => Err(AdventError::UnknownTileError(tile))
	}
}
