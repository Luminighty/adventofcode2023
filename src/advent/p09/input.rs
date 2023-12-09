use std::num::ParseIntError;

use super::{error::AdventError, oasis::{Report, Reading}};

pub fn get_input() -> Result<Vec<Report>, AdventError> {
	let content = std::fs::read_to_string("inp/09.txt")
		.map_err(|_| AdventError::FileReadError)?;
	let reports = content
		.lines()
		.map(|line| 
			line.split(' ')
				.map(|n| n.parse())
				.collect()
		)
		.collect::<Result<Vec<Vec<Reading>>, ParseIntError>>()?;
	Ok(
		reports
		.iter()
		.map(|r| Report::new(r.clone()))
		.collect()
	)
}
