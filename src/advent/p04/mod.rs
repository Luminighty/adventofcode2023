use std::collections::HashSet;

use self::input::get_input;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 4: Scratchcards";

mod error;
mod input;
mod card;


fn get_matches(card: &card::Card) -> u32 {
	let winning: HashSet<u32> = HashSet::from_iter(card.winning.iter().cloned());
	let mut matches = 0;
	for actual in &card.actual {
		if winning.contains(&actual) {
			matches += 1;
		}
	}
	matches
}

fn get_points(card: &card::Card) -> u32 {
	let matches = get_matches(card);
	if matches == 0 {
		0
	} else {
		2_u32.pow(matches - 1)
	}
}

pub fn solve_a() {
	let cards = get_input().unwrap();

	let mut sum = 0;
	for card in &cards {
		sum += get_points(card);
	}
	println!("Total points: {}", sum);
}

pub fn solve_b() {
	let cards = get_input().unwrap();
	let mut copies = vec![1; cards.len()];
	let mut sum = 0;
	for card in &cards {
		let current_copies = copies[(card.id - 1) as usize];
		sum += current_copies;
		let matches = get_matches(card);
		for i in 0..matches {
			let index = (card.id + i) as usize;
			if index < copies.len() {
				copies[index] += current_copies;
			}
		}
	}
	println!("Total Cards: {}", sum);
}
