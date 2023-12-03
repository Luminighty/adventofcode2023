use super::{error::AdventError, engine::Part};

type Input = Vec<Vec<Part>>;

pub fn get_input() -> Result<Input, AdventError> {
	let content = std::fs::read_to_string("inp/03.txt")
		.map_err(|_| AdventError::FileReadError)?;

	let engine = content
			.lines()
			.map(|line|
				line
					.chars()
					.map(Part::from)
					.collect()
			).collect();
	Ok(engine)
}
