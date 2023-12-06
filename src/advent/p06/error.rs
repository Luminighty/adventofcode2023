use std::num::ParseIntError;

#[derive(Debug)]
pub enum AdventError {
	FileReadError,
	ParseIntError(ParseIntError),
	FileFormatError,
}

impl From<ParseIntError> for AdventError {
	fn from(value: ParseIntError) -> Self {
		AdventError::ParseIntError(value)
	}
}