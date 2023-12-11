#[derive(Debug)]
pub enum AdventError {
	FileReadError,
	UnknownTile(char),
}
