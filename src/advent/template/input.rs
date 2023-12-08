use super::error::AdventError;

pub fn get_input() -> Result<(), AdventError> {
	let content = std::fs::read_to_string("inp/.txt")
		.map_err(|_| AdventError::FileReadError)?;
	Ok(())
}
