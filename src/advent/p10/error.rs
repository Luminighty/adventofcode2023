#[derive(Debug)]
pub enum AdventError {
	FileReadError,
	UnknownPipe(char)
}
