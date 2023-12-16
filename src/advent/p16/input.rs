use super::{error::AdventError, map::{Tile, Grid, Map}};

pub fn get_input() -> Result<Map, AdventError> {
	let content = std::fs::read_to_string("inp/16.txt")
		.map_err(|_| AdventError::FileReadError)?;
	let grid = content.lines()
		.map(parse_line)
		.collect::<Result<Grid, AdventError>>()?;
	Ok(Map::new(grid))
}

fn parse_line(s: &str) -> Result<Vec<Tile>, AdventError> {
	s.chars().map(parse_tile).collect()
}

fn parse_tile(c: char) -> Result<Tile, AdventError> {
	match c {
		'.' => Ok(Tile::Space),
		'/' => Ok(Tile::Mirror),
		'\\' => Ok(Tile::BackMirror),
		'|' => Ok(Tile::VerticalSplitter),
		'-' => Ok(Tile::HorizontalSplitter),
		_ => Err(AdventError::UnknownTileError(c))
	}
}