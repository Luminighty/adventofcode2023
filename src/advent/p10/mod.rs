use self::pipe::{Maze, Direction, Pipe};

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 10: Pipe Maze";

mod error;
mod input;
mod pipe;

fn init_maze() -> Option<((usize, usize), Direction, Maze)> {
	let mut maze = input::get_input().unwrap();
	let (start_x, start_y) = maze.find_start()?;
	maze.fix_pipe(start_x, start_y);
	let start_pipe = maze.get(start_x, start_y)?;
	let d = match start_pipe {
		Pipe::Horizontal => Some(Direction::East),
		Pipe::Vertical => Some(Direction::North),
		Pipe::NE => Some(Direction::North),
		Pipe::NW => Some(Direction::North),
		Pipe::SE => Some(Direction::South),
		Pipe::SW => Some(Direction::South),
		_ => None
	}?;
	Some(((start_x, start_y), d, maze))
}

pub fn solve_a() {
	if let Some((start, direction, maze)) = init_maze() {
		let path = pipe::explore(&maze, start, direction);
		println!("Farthest distance: {}", path.len() / 2);
	} else {
		println!("Couldn't initialize the maze")
	}
}

pub fn solve_b() {
	if let Some((start, direction, maze)) = init_maze() {
		let path = pipe::explore(&maze, start, direction);
		println!("Enclosed tiles: {}", pipe::enclosed(&maze, path));
	} else {
		println!("Couldn't initialize the maze")
	}

}
