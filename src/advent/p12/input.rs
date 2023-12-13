use std::{num::ParseIntError, collections::VecDeque};

use super::{error::AdventError, parts::{Condition, Record}};

pub fn get_input() -> Result<Vec<Record>, AdventError> {
	let content = std::fs::read_to_string("inp/12.txt")
		.map_err(|_| AdventError::FileReadError)?;
	content
		.lines()
		.map(parse_record)
		.collect()
}

fn parse_record(line: &str) -> Result<Record, AdventError> {
	let (conditions, groups) = line.split_once(' ').ok_or(AdventError::FileFormatError)?;
	let conditions = parse_conditions(conditions)?;
	let groups = parse_groups(groups)?;
	Ok(Record::new(conditions, groups))
}

fn parse_groups(groups: &str) -> Result<VecDeque<usize>, ParseIntError> {
	groups
		.split(',')
		.map(|s| s.parse())
		.collect()
}

fn parse_conditions(cond: &str) -> Result<Vec<Condition>, AdventError> {
	cond.chars().map(parse_condition).collect()
}

fn parse_condition(c: char) -> Result<Condition, AdventError> {
	match c {
		'.' => Ok(Condition::Operational),
		'#' => Ok(Condition::Damaged),
		'?' => Ok(Condition::Unknown),
		_ => Err(AdventError::UnknownCondition(c))
	}
}