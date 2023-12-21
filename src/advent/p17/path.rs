use std::collections::HashSet;

use super::heat::{Item, Map};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
	Up, Left, Right, Down
}

impl Direction {
	pub fn delta(&self) -> (isize, isize) {
		match self {
			Direction::Up => (0, -1),
			Direction::Down => (0, 1),
			Direction::Left => (-1, 0),
			Direction::Right => (1, 0),
		}
	}

	pub fn left(&self) -> Self {
		match self {
			Direction::Up => Direction::Left,
			Direction::Left => Direction::Down,
			Direction::Down => Direction::Right,
			Direction::Right => Direction::Up,
		}
	}

	pub fn right(&self) -> Self {
		match self {
			Direction::Up => Direction::Right,
			Direction::Left => Direction::Up,
			Direction::Down => Direction::Left,
			Direction::Right => Direction::Down,
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct State {
	x: isize,
	y: isize,
	heatloss: Item,
	same_direction: usize,
	direction: Direction,
}

impl State {
	pub fn new(
		x: isize, 
		y: isize, 
		heatloss: Item, 
		same_direction: usize, 
		direction: Direction
	) -> Self {
		Self { x, y, heatloss, same_direction, direction }
	}

	pub fn left(&self) -> Self {
		let new_direction = self.direction.left();
		let (dx, dy) = new_direction.delta();
		Self {
			x: self.x + dx,
			y: self.y + dy,
			direction: new_direction,
			same_direction: 0,
			heatloss: self.heatloss
		}
	}

	pub fn right(&self) -> Self {
		let new_direction = self.direction.right();
		let (dx, dy) = new_direction.delta();
		Self {
			x: self.x + dx,
			y: self.y + dy,
			direction: new_direction,
			same_direction: 0,
			heatloss: self.heatloss
		}
	}

	pub fn forward(&self) -> Self {
		let (dx, dy) = self.direction.delta();
		Self {
			x: self.x + dx,
			y: self.y + dy,
			direction: self.direction,
			same_direction: self.same_direction + 1,
			heatloss: self.heatloss
		}
	}
}

fn next(queue: &mut Vec<State>) -> Option<State> {
	let mut min = 0;
	for i in 1..queue.len() {
		if queue[min].heatloss < queue[i].heatloss {
			continue;
		}
		if queue[min].heatloss == queue[i].heatloss {
			let min_distance = queue[min].x + queue[min].y;
			let i_distance = queue[i].x + queue[i].y;
			if min_distance < i_distance {
				min = i;
			}
			continue;
		}
		min = i;
	}
	if min < queue.len() {
		Some(queue.swap_remove(min))
	} else {
		None
	}
}

fn contains(visited: &HashSet<State>, state: &State) -> bool {
	for other in visited {
		if other.x != state.x || other.y != state.y { continue; }
		if other.direction != state.direction { continue; }
		// if other.same_direction >= state.same_direction { continue; }
		if other.heatloss >= state.heatloss { continue; }
		return true;
	}
	false
}

const MAX_STEP_FORWARD: usize = 3;
fn apply_state(map: &Map, mut state: State, visited: &HashSet<State>) -> Option<State> {
	if state.same_direction >= MAX_STEP_FORWARD { return None; }
	let tile = map.get(state.x, state.y)?;
	state.heatloss += *tile;
	if contains(&visited, &state) {
		None
	} else {
		Some(state)
	}
}

pub fn find(map: Map, start: (isize, isize), target: (isize, isize)) -> Option<Item> {
	let init_state = State::new(start.0, start.1, 0, 0, Direction::Right);
	let mut visited = HashSet::new();
	let mut queue = Vec::new();
	queue.push(init_state);
	while let Some(next) = next(&mut queue) {
		if next.x == target.0 && next.y == target.1 {
			return Some(next.heatloss);
		}
		if let Some(state) = apply_state(&map, next.forward(), &visited) { 
			queue.push(state);
		}
		if let Some(state) = apply_state(&map, next.left(), &visited) { 
			queue.push(state);
		}
		if let Some(state) = apply_state(&map, next.right(), &visited) { 
			queue.push(state);
		}
		visited.insert(next);
	}
	None
}