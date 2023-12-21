#[derive(Debug)]
pub enum AdventError {
	FileReadError,
	ParseIntError(char)
}