use crate::advent::p06::race::Race;

use self::{input::get_input, race::Unit};

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 6: Wait For It";

mod error;
mod input;
mod race;

pub fn solve_a() {
	let races = get_input().unwrap();
	let mut total = 1;
	for race in races {
		let mut wins = 0;
		for hold in 1..race.time {
			if race.total_distance(hold) > race.distance {
				wins += 1;
			}
		}
		total *= wins;
	}
	println!("Margin: {}", total);
}

fn find_race(race: &Race, iter: Vec<Unit>) -> Option<Unit> {
	for hold in iter {
		if race.total_distance(hold) > race.distance {
			return Some(hold)
		}
	}
	None
}

pub fn solve_b() {
	let races = get_input().unwrap();
	let race = Race::merge(races);
	let min = find_race(&race, (0..race.time).into_iter().collect());
	let max = find_race(&race, (0..race.time).rev().collect());
	println!("min: {:?}; max: {:?}", min, max);
	match (min, max) {
		(Some(min), Some(max)) => println!("Total Hold Margin: {}", max - min + 1),
		_ => {}
	}
}

