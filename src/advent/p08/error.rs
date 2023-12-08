#[derive(Debug)]
pub enum AdventError {
	FileReadError,
	FileFormatError,
	NodeFormatError,
	UnknownTurn,
}
