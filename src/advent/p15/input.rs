use super::{error::AdventError, hash::Operation};

pub fn get_input() -> Result<Vec<String>, AdventError> {
	let content = std::fs::read_to_string("inp/15.txt")
		.map_err(|_| AdventError::FileReadError)?;
	Ok(content.split(',').map(|s| s.to_owned()).collect())
}


pub fn get_steps() -> Result<Vec<Operation>, AdventError> {
	let steps = get_input()?;
	steps.iter()
		.map(parse_step)
		.collect()
}

fn parse_step(step: &String) -> Result<Operation, AdventError> {
	if let Some((label, lens)) = step.split_once('=') {
		return Ok(Operation::Add(label.to_owned(), lens.parse()?))
	}
	if let Some((label, _)) = step.split_once('-') {
		return Ok(Operation::Remove(label.to_owned()));
	}
	Err(AdventError::UnknownStepError(step.to_owned()))
}