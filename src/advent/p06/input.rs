use std::num::ParseIntError;

use super::{error::AdventError, race::{Unit, Race}};

pub fn get_input() -> Result<Vec<Race>, AdventError> {
	let content = std::fs::read_to_string("inp/06.txt")
		.map_err(|_| AdventError::FileReadError)?;
	let mut lines = content.lines().take(2);

	let time = lines.next().ok_or(AdventError::FileFormatError)?;
	let distance = lines.next().ok_or(AdventError::FileFormatError)?;
	let time = read_line(time, "Time:")?;
	let distance = read_line(distance, "Distance:")?;
	Ok(
		time.iter()
			.zip(distance)
			.map(|(time, distance)| Race::new(*time, distance))
			.collect()
	)
}

pub fn read_line(line: &str, prefix: &str) -> Result<Vec<Unit>, ParseIntError> {
	line.replace(prefix, "")
		.split(' ')
		.filter(|s| !s.is_empty())
		.map(|n| n.parse())
		.collect()
}