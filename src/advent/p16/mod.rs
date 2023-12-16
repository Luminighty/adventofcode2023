use std::collections::{HashMap, HashSet};

use self::map::{Beam, Map, Action, Direction};

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 16: The Floor Will Be Lava";

mod error;
mod input;
mod map;

fn step(map: &Map, beams: &mut Vec<Beam>, i: usize) {
	let beam = &mut beams[i];
	
	let tile = map.get(beam.x, beam.y).unwrap_or(&map::Tile::Space);
	match tile.apply(&beam.direction) {
    Action::None => {
			beam.step();
		},
    Action::NewDirection(d) => {
			beam.direction = d;
			beam.step();
		},
    Action::VerticalSplit => {
			let mut other = beam.clone();
			other.direction = Direction::Up;
			beam.direction = Direction::Down;
			other.step();
			beam.step();
			beams.push(other)
		},
    Action::HorizontalSplit => {
			let mut other = beam.clone();
			other.direction = Direction::Left;
			beam.direction = Direction::Right;
			other.step();
			beam.step();
			beams.push(other)
		},
	}
}

fn energized_fields(map: &Map, start: Beam) -> usize {
	let mut beams = vec![start];
	let mut energized: HashMap<(isize, isize), HashSet<Direction>> = HashMap::new();

	while beams.len() > 0 {
		// energize
		for beam in &beams {
			if let Some(dirs) = energized.get_mut(&(beam.x, beam.y)) {
				dirs.insert(beam.direction.clone());
			} else {
				let mut set = HashSet::new();
				set.insert(beam.direction.clone());
				energized.insert((beam.x, beam.y), set);
			}
		}
		// Step
		for i in (0..beams.len()).rev() {
			step(&map, &mut beams, i);
		}
		// Remove already seen/ oob beams
		for i in (0..beams.len()).rev() {
			let beam = &beams[i];
			let in_bounds = map.in_bounds(beam.x, beam.y);
			let is_energized = energized.get(&(beam.x, beam.y))
				.map(|tile| tile.contains(&beam.direction))
				.unwrap_or(false);
			if in_bounds && !is_energized { continue; }
			beams.remove(i);
		}
	}
	energized.len()
}

pub fn solve_a() {
	let map = input::get_input().unwrap();
	println!("Energized tiles: {}", energized_fields(&map, Beam::start()));
}

pub fn solve_b() {
	let map = input::get_input().unwrap();

	let mut max = 0;
	for x in 0..map.width() {
		max = max.max(energized_fields(&map, Beam::new(Direction::Down, x as isize, 0)));
		max = max.max(energized_fields(&map, Beam::new(Direction::Up, x as isize, map.height() as isize - 1)));
	}
	for y in 0..map.height() {
		max = max.max(energized_fields(&map, Beam::new(Direction::Right, 0, y as isize)));
		max = max.max(energized_fields(&map, Beam::new(Direction::Left, map.width() as isize - 1, y as isize)));
	}
	println!("Best energized tiles: {}", max);
}
