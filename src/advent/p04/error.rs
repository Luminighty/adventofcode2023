use std::num::ParseIntError;

#[derive(Debug)]
pub enum AdventError {
	FileReadError,
	IncorrectCardFormat,
	ParseIntError(ParseIntError)
}

impl From<ParseIntError> for AdventError {
	fn from(value: ParseIntError) -> Self {
		AdventError::ParseIntError(value)
	}
}