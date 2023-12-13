#[derive(Debug)]
pub enum AdventError {
	FileReadError,
	UnknownTileError(char),
}
