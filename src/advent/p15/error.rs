use std::num::ParseIntError;

#[derive(Debug)]
pub enum AdventError {
	FileReadError,
	UnknownStepError(String),
	ParseIntError(ParseIntError),
}

impl From<ParseIntError> for AdventError {
	fn from(value: ParseIntError) -> Self {
		AdventError::ParseIntError(value)
	}
}