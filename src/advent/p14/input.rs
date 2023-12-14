use super::{error::AdventError, dish::{Dish, Tile, Grid}};

pub fn get_input() -> Result<Dish, AdventError> {
	let content = std::fs::read_to_string("inp/14.txt")
		.map_err(|_| AdventError::FileReadError)?;
	parse_dish(&content)
}

fn parse_dish(dish: &str) -> Result<Dish, AdventError> {
	let dish = dish
		.lines()
		.map(|line|
			line
				.chars()
				.map(parse_tile)
				.collect()
		)
		.collect::<Result<Grid, AdventError>>()?;
	Ok(Dish::new(dish))
}

fn parse_tile(c: char) -> Result<Tile, AdventError> {
	match c {
		'O' => Ok(Tile::RoundRock),
		'#' => Ok(Tile::CubeRock),
		'.' => Ok(Tile::Space),
		_ => Err(AdventError::UnknownTile(c))
	}
}