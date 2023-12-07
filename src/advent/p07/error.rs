use std::num::ParseIntError;

#[derive(Debug)]
pub enum AdventError {
	FileReadError,
	FileFormatError,
	ParseIntError(ParseIntError),
	UnknownCard(char),
	MissingCard,
}


impl From<ParseIntError> for AdventError {
	fn from(value: ParseIntError) -> Self {
		AdventError::ParseIntError(value)
	}
}