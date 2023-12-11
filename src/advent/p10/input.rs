use super::{error::AdventError, pipe::{Pipe, Maze}};

pub fn get_input() -> Result<Maze, AdventError> {
	let content = std::fs::read_to_string("inp/10.txt")
		.map_err(|_| AdventError::FileReadError)?;

	let maze = content
		.lines()
		.map(|line|
			line.chars()
				.map(parse_pipe)
				.collect()
		).collect::<Result<Vec<Vec<Pipe>>, AdventError>>()?;
	Ok(Maze::new(maze))
}

pub fn parse_pipe(c: char) -> Result<Pipe, AdventError> {
	match c {
		'|' => Ok(Pipe::Vertical),
		'-' => Ok(Pipe::Horizontal),
		'L' => Ok(Pipe::NE),
		'J' => Ok(Pipe::NW),
		'7' => Ok(Pipe::SW),
		'F' => Ok(Pipe::SE),
		'.' => Ok(Pipe::Ground),
		'S' => Ok(Pipe::Start),
		_ => Err(AdventError::UnknownPipe(c))
	}
}