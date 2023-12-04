use std::num::ParseIntError;

use super::{error::AdventError, card::Card};

pub fn get_input() -> Result<Vec<Card>, AdventError> {
	let content = std::fs::read_to_string("inp/04.txt").map_err(|_| AdventError::FileReadError)?;
	content
		.lines()
		.map(parse_card)
		.collect()
}

fn parse_card(line: &str) -> Result<Card, AdventError> {
	let (id, numbers) = line.split_once(':').ok_or(AdventError::IncorrectCardFormat)?;
	let id = id.replace("Card ", "").trim().parse()?;
	let (winning, actual) = numbers.split_once('|').ok_or(AdventError::IncorrectCardFormat)?;
	let winning = parse_numbers(winning)?;
	let actual = parse_numbers(actual)?;

	Ok(Card::new(id, winning, actual))
}

fn parse_numbers(numbers: &str) -> Result<Vec<u32>, ParseIntError> {
	numbers
		.trim()
		.split(' ')
		.filter(|s| !s.is_empty())
		.map(str::trim)
		.map(|n| n.parse::<u32>())
		.collect::<Result<Vec<u32>, ParseIntError>>()
}
