use std::num::ParseIntError;

use super::game::{Game, Pull};


#[derive(Debug)]
pub enum AdventError {
	FileReadError,
	IncorrectGameFormat,
	IncorrectPullFormat,
	IncorrectColor,
	ParseIntError(ParseIntError)
}

pub fn get_input() -> Result<Vec<Game>, AdventError> {
	let content = std::fs::read_to_string("inp/02.txt").map_err(|_| AdventError::FileReadError)?;
	content
		.lines()
		.map(parse_game)
		.collect()
}

fn parse_game(line: &str) -> Result<Game, AdventError> {
	let (game, pulls) = line.split_once(':').ok_or(AdventError::IncorrectGameFormat)?;
	let id: u32 = game.replace("Game ", "").parse()?;
	let pulls: Result<Vec<Pull>, AdventError> = pulls.split(";").map(parse_pulls).collect();
	let pulls = pulls?;
	Ok(Game::new(id, pulls))
}

fn parse_pulls(pull_str: &str) -> Result<Pull, AdventError> {
	let mut pull = Pull::new();
	for cube in pull_str.split(',') {
		let (amount, color) = cube.trim().split_once(' ').ok_or(AdventError::IncorrectPullFormat)?;
		let amount: u32 = amount.parse()?;
		pull = match color {
			"red" => pull.pull_red(amount),
			"green" => pull.pull_green(amount),
			"blue" => pull.pull_blue(amount),
			_ => { return Err(AdventError::IncorrectColor);}
		}
	}
	Ok(pull)
}

impl From<ParseIntError> for AdventError {
	fn from(value: ParseIntError) -> Self {
		AdventError::ParseIntError(value)
	}
}