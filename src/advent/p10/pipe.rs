use std::{fmt::Debug, collections::{HashSet, VecDeque}};

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
	North, East, South, West
}

impl Direction {
	pub fn apply(&self, x: usize, y: usize) -> Option<(usize, usize)> {
		match self {
			Direction::East => Some((x + 1, y)),
			Direction::South => Some((x, y + 1)),
			Direction::West if x > 0 => Some((x - 1, y)),
			Direction::North if y > 0 => Some((x, y - 1)),
			_ => None
		}
	}
}

#[derive(PartialEq, Eq)]
pub enum Pipe {
	Horizontal,
	Vertical,
	NE,
	NW,
	SE,
	SW,
	Ground,
	Start,
}


impl Debug for Pipe {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Pipe::Horizontal => '-',
			Pipe::Vertical => '|',
			Pipe::NE => 'L',
			Pipe::NW => 'J',
			Pipe::SE => '7',
			Pipe::SW => 'F',
			Pipe::Ground => '.',
			Pipe::Start => 'S',
		})
	}
}

impl Pipe {
	pub fn step(&self, from: &Direction) -> Option<Direction> {
		match (self, from) {
			(Pipe::Horizontal, Direction::East) => Some(Direction::East),
			(Pipe::Horizontal, Direction::West) => Some(Direction::West),
			(Pipe::Horizontal, _) => None,

			(Pipe::Vertical, Direction::North) => Some(Direction::North),
			(Pipe::Vertical, Direction::South) => Some(Direction::South),
			(Pipe::Vertical, _) => None,

			(Pipe::NE, Direction::South) => Some(Direction::East),
			(Pipe::NE, Direction::West) => Some(Direction::North),
			(Pipe::NE, _) => None,

			(Pipe::NW, Direction::East) => Some(Direction::North),
			(Pipe::NW, Direction::South) => Some(Direction::West),
			(Pipe::NW, _) => None,
			
			(Pipe::SE, Direction::North) => Some(Direction::East),
			(Pipe::SE, Direction::West) => Some(Direction::South),
			(Pipe::SE, _) => None,
			
			(Pipe::SW, Direction::North) => Some(Direction::West),
			(Pipe::SW, Direction::East) => Some(Direction::South),
			(Pipe::SW, _) => None,
			
			(Pipe::Ground, _) => None,
			(Pipe::Start, _) => None,
		}
	}

	pub fn is_solid(&self) -> bool {
		self != &Pipe::Ground
	}

	pub fn is_facing(&self, direction: Direction) -> bool {
		match self {
			Pipe::Vertical => direction == Direction::North || direction == Direction::South,
			Pipe::Horizontal => direction == Direction::West || direction == Direction::East,
			Pipe::NE => direction == Direction::North || direction == Direction::East,
			Pipe::NW => direction == Direction::North || direction == Direction::West,
			Pipe::SE => direction == Direction::South || direction == Direction::East,
			Pipe::SW => direction == Direction::South || direction == Direction::West,
			_ => false
		}
	}
}

pub struct Maze {
	maze: Vec<Vec<Pipe>>
}

impl Maze {
	pub fn new(maze: Vec<Vec<Pipe>>) -> Self {
		Self { maze }
	}

	pub fn find_start(&self) -> Option<(usize, usize)> {
		for y in 0..self.maze.len() {
			for x in 0..self.maze[y].len() {
				if self.maze[y][x] == Pipe::Start {
					return Some((x, y));
				}
			}
		}
		None
	}

	pub fn get(&self, x: usize, y: usize) -> Option<&Pipe> {
		self.maze.get(y).map(|l| l.get(x)).flatten()
	}

	pub fn fix_pipe(&mut self, x: usize, y: usize) {
		let top = y > 0 && self.get(x, y - 1).map(|pipe| pipe.is_facing(Direction::South)).unwrap_or(false);
		let left = x > 0 && self.get(x - 1, y).map(|pipe| pipe.is_facing(Direction::East)).unwrap_or(false);
		let bottom = self.get(x, y + 1).map(|pipe| pipe.is_facing(Direction::North)).unwrap_or(false);
		let right = self.get(x + 1, y).map(|pipe| pipe.is_facing(Direction::West)).unwrap_or(false);

		let new_pipe = match (top, left, bottom, right) {
			(true, false, true, false) => Some(Pipe::Vertical),
			(false, true, false, true) => Some(Pipe::Horizontal),
			(true, true, false, false) => Some(Pipe::NW),
			(false, true, true, false) => Some(Pipe::SW),
			(false, false, true, true) => Some(Pipe::SE),
			(true, false, false, true) => Some(Pipe::NE),
			_ => None
		};
		if let Some(new_pipe) = new_pipe {
			self.maze[y][x] = new_pipe;
		} else {
			println!("Couldn't figure out the actual pipe at ({}, {})", x, y);
			println!("top {}; left: {}, bottom: {}, right: {}", top, left, bottom, right);
			println!("top {:?}; left: {:?}, bottom: {:?}, right: {:?}", self.get(x, y - 1), self.get(x - 1, y), self.get(x, y + 1), self.get(x + 1, y));
		}
	}
}

impl Debug for Maze {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for line in &self.maze {
			for pipe in line {
				write!(f, "{:?}", pipe)?;
			}
			writeln!(f, "")?;
		}
		Ok(())
	}
}

pub fn explore(maze: &Maze, start: (usize, usize), direction: Direction) -> Vec<(usize, usize)> {
	let mut path = Vec::new();
	let mut x = start.0;
	let mut y = start.1;
	let mut direction = direction;
	loop {
		path.push((x, y));
		if let Some((new_x, new_y)) = direction.apply(x, y) {
			x = new_x;
			y = new_y;
		}
		let new_direction = maze.get(x, y).and_then(|pipe| pipe.step(&direction));
		if let Some(new_direction) = new_direction {
			direction = new_direction;
		}

		if x == start.0 && y == start.1 {
			break;
		}
	}
	path
}

pub type Pos = (usize, usize);
pub type Path = Vec<Pos>;

fn bounds(path: &Path) -> (Pos, Pos) {
	let min_x = path.iter().map(|v| v.0).min().unwrap_or(usize::MAX);
	let min_y = path.iter().map(|v| v.1).min().unwrap_or(usize::MAX);
	let max_x = path.iter().map(|v| v.0).max().unwrap_or(usize::MIN);
	let max_y = path.iter().map(|v| v.1).max().unwrap_or(usize::MIN);
	((min_x, min_y), (max_x, max_y))
}


fn explore_open_area(maze: &Maze, x: usize, y: usize, (min, max): &(Pos, Pos)) -> HashSet<Pos> {
	let mut open_pos = HashSet::new();
	let mut queue = VecDeque::new();

	queue.push_back((x, y));
	while let Some((x, y)) = queue.pop_front() {
		open_pos.insert((x, y));

		if x > min.0 && maze.get(x - 1, y).is_some_and(|p| !p.is_solid()) && !open_pos.contains(&(x - 1, y)) {
			queue.push_back((x - 1, y))
		}
		if y > min.1 && maze.get(x, y - 1).is_some_and(|p| !p.is_solid()) && !open_pos.contains(&(x, y - 1)) {
			queue.push_back((x, y - 1))
		}
		if x < max.0 && maze.get(x + 1, y).is_some_and(|p| !p.is_solid()) && !open_pos.contains(&(x + 1, y)) {
			queue.push_back((x + 1, y))
		}
		if y < max.1 && maze.get(x, y + 1).is_some_and(|p| !p.is_solid()) && !open_pos.contains(&(x, y + 1)) {
			queue.push_back((x, y + 1))
		}

		
		if x > min.0 && y > min.1 && maze.get(x - 1, y - 1).is_some_and(|p| !p.is_solid()) && !open_pos.contains(&(x - 1, y - 1)) {
			queue.push_back((x - 1, y - 1))
		}
		if x < max.0 && y < max.1 && maze.get(x + 1, y + 1).is_some_and(|p| !p.is_solid()) && !open_pos.contains(&(x + 1, y + 1)) {
			queue.push_back((x + 1, y + 1))
		}

		if x < max.0 && y > min.1 && maze.get(x + 1, y - 1).is_some_and(|p| !p.is_solid()) && !open_pos.contains(&(x + 1, y - 1)) {
			queue.push_back((x + 1, y - 1))
		}
		if x > min.0 && y < max.1 && maze.get(x - 1, y + 1).is_some_and(|p| !p.is_solid()) && !open_pos.contains(&(x - 1, y + 1)) {
			queue.push_back((x - 1, y + 1))
		}
	}

	open_pos
}


pub fn enclosed(maze: &Maze, path: Vec<(usize, usize)>) -> usize {
	let mut open_area = HashSet::new();
	let (min, max) = bounds(&path);
	for x in min.0..=max.0 {
		if maze.get(x, min.1).is_some_and(|p| !p.is_solid()) && !open_area.contains(&(x, min.1)) {
			open_area = open_area.union(&explore_open_area(maze, x, min.1, &(min, max))).map(|p| *p).collect();
		}
		if maze.get(x, max.1).is_some_and(|p| !p.is_solid()) && !open_area.contains(&(x, max.1)) {
			open_area = open_area.union(&explore_open_area(maze, x, max.1, &(min, max))).map(|p| *p).collect();
		}
	}
	for y in min.1..=max.1 {
		if maze.get(min.0, y).is_some_and(|p| !p.is_solid()) && !open_area.contains(&(min.0, y)) {
			open_area = open_area.union(&explore_open_area(maze, min.0, y, &(min, max))).map(|p| *p).collect();
		}
		if maze.get(max.0, y).is_some_and(|p| !p.is_solid()) && !open_area.contains(&(max.0, y)) {
			open_area = open_area.union(&explore_open_area(maze, max.0, y, &(min, max))).map(|p| *p).collect();
		}
	}

	let mut pipe_count = 0;
	for y in min.1..=max.1 {
		for x in min.0..=max.0 {
			if let Some(pipe) = maze.get(x, y) {
				if pipe.is_solid() {
					pipe_count += 1;
					print!("{:?}", pipe);
				} else {
					print!("{}", if open_area.contains(&(x, y)) { "O" } else { "I" });
				}
			}
		}
		println!();
	}
	let tiles = (max.0 - min.0 + 1) * (max.1 - min.1 + 1);
	tiles - pipe_count - open_area.len()
}