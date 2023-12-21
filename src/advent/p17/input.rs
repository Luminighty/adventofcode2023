use super::{error::AdventError, heat::{Map, Item, Grid}};

pub fn get_input() -> Result<Map, AdventError> {
	let content = std::fs::read_to_string("inp/17.txt")
		.map_err(|_| AdventError::FileReadError)?;
	let map = content.lines()
		.map(parse_line)
		.collect::<Result<Grid, AdventError>>()?;
	Ok(Map::new(map))
}

fn parse_line(line: &str) -> Result<Vec<Item>, AdventError> {
	line.chars()
		.map(parse_digit)
		.collect()
}

fn parse_digit(c: char) -> Result<Item, AdventError> {
	match c {
		'0' => Ok(0),
		'1' => Ok(1),
		'2' => Ok(2),
		'3' => Ok(3),
		'4' => Ok(4),
		'5' => Ok(5),
		'6' => Ok(6),
		'7' => Ok(7),
		'8' => Ok(8),
		'9' => Ok(9),
		_ => Err(AdventError::ParseIntError(c))
	}
}