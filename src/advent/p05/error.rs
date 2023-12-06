use std::num::ParseIntError;

#[derive(Debug)]
pub enum AdventError {
	FileReadError,
	FileFormatError,
	ParseIntError(ParseIntError),
	MapFormatError(String),
	RangeFormatError(String),
}

impl From<ParseIntError> for AdventError {
	fn from(value: ParseIntError) -> Self {
		AdventError::ParseIntError(value)
	}
}