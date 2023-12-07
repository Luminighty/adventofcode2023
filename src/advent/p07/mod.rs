use crate::advent::p07::cards::Bid;

use self::cards::Game;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 7: Camel Cards";

mod error;
mod input;
mod cards;

fn get_winnings(games: &mut Vec<Game>) -> Bid {
	games.reverse();
	let mut winnings = 0;
	for (rank, game) in games.iter().enumerate() {
		winnings += (rank + 1) as Bid * game.bid;
	}
	winnings
}

pub fn solve_a() {
	let mut games = input::get_input().unwrap();
	games.sort_by(Game::cmp);
	println!("Total Winnings: {}", get_winnings(&mut games));
}

pub fn solve_b() {
	let mut games = input::get_input().unwrap();
	games = games.iter().map(Game::to_joker).collect();
	games.sort_by(Game::cmp);
	println!("Total Winning Variation: {}", get_winnings(&mut games));
}
