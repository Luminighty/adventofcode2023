use std::cmp::Ordering;

use self::dish::Pos;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 14: Parabolic Reflector Dish";

mod error;
mod input;
mod dish;
mod state;

enum Tilt {
	North, West, South, East
}

impl Tilt {
	pub fn delta(&self) -> (isize, isize) {
		match self {
			Tilt::North => (0, -1),
			Tilt::South => (0, 1),
			Tilt::West => (-1, 0),
			Tilt::East => (1, 0),
		}
	}
}

fn north_sort(a: &Pos, b: &Pos) -> Ordering {
	match a.1.cmp(&b.1) {
		Ordering::Equal => a.0.cmp(&b.0),
		ord => ord
	}
}

fn south_sort(a: &Pos, b: &Pos) -> Ordering {
	north_sort(a, b).reverse()
}

fn west_sort(a: &Pos, b: &Pos) -> Ordering {
	match a.0.cmp(&b.0) {
		Ordering::Equal => a.1.cmp(&b.1),
		ord => ord
	}
}

fn east_sort(a: &Pos, b: &Pos) -> Ordering {
	west_sort(a, b).reverse()
}

fn tilt_step(
	round: &mut Vec<Pos>, 
	cube: &Vec<Pos>, 
	width: usize, 
	height: usize, 
	tilt: &Tilt,
	i: usize,
) {
	loop {
		let (x, y) = round[i];
		let (dx, dy) = tilt.delta();
		let new_x = x as isize + dx;
		let new_y = y as isize + dy;
		if new_x < 0 || new_y < 0 || new_x >= width as isize || new_y >= height as isize {
			return;
		}
		let new_pos = (new_x as usize, new_y as usize);
		if cube.contains(&new_pos) { return; }
		if round.contains(&new_pos) { return; }
		round[i] = new_pos;
	}
}

fn tilt(round: &mut Vec<Pos>, cube: &Vec<Pos>, width: usize, height: usize, tilt: Tilt) {
	match tilt {
		Tilt::North => round.sort_by(north_sort),
		Tilt::South => round.sort_by(south_sort),
		Tilt::West => round.sort_by(west_sort),
		Tilt::East => round.sort_by(east_sort),
	}
	for i in 0..round.len() {
		tilt_step(round, cube, width, height, &tilt, i);
	}
}

fn cycle(round: &mut Vec<Pos>, cube: &Vec<Pos>, width: usize, height: usize) {
	tilt(round, cube, width, height, Tilt::North);
	tilt(round, cube, width, height, Tilt::West);
	tilt(round, cube, width, height, Tilt::South);
	tilt(round, cube, width, height, Tilt::East);
}

fn total_load(round: &Vec<Pos>, height: usize) -> usize {
	let mut sum = 0;
	for (_, y) in round {
		let weight = height - y;
		sum += weight;
	}
	sum
}

pub fn solve_a() {
	let dish = input::get_input().unwrap();
	
	let mut round = dish.round_rocks();
	let cube = dish.cube_rocks();
	
	let height = dish.height();
	let width = dish.width();

	tilt(&mut round, &cube, width, height, Tilt::North);
	println!("Total load: {}", total_load(&round, height));
}


fn extra_cycles(start: usize, repeat: usize, target: usize) -> usize {
	let step = repeat - start;
	let mut current = repeat;
	while current + step <= target {
		current += step;
	}
	target - current - 1
}

pub fn solve_b() {
	let dish = input::get_input().unwrap();
	
	let mut round = dish.round_rocks();
	let cube = dish.cube_rocks();
	
	let height = dish.height();
	let width = dish.width();

	let mut states = vec![state::State::new(&round)];
	for i in 0..1_000_000_000 {
		cycle(&mut round, &cube, width, height);
		let new_state = state::State::new(&round);
		if states.contains(&new_state) {
			let (other, _) = states.iter().enumerate().find(|&(_, state)| state == &new_state).unwrap();
			let cycles = extra_cycles(other, i, 1_000_000_000);
			for _ in 0..cycles {
				cycle(&mut round, &cube, width, height);
			}
			println!("Total Load after 1B cycles: {}", total_load(&round, height));
			return;
		} else {
			states.push(new_state)
		}
	}
}