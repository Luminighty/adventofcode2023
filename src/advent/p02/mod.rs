use self::input::get_input;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 2: Cube Conundrum";

mod game;
mod input;

fn is_possible(pull: &game::Pull) -> bool {
	pull.red <= 12 && pull.green <= 13 && pull.blue <= 14
}

pub fn solve_a() {
	let games = get_input().unwrap();
	let mut sum = 0;
	for game in games {
		let max = game.max_pulls();
		if is_possible(&max) {
			sum += game.id;
		}
	}
	println!("Sum of possible games: {}", sum);
}

pub fn solve_b() {
	let games = get_input().unwrap();
	let mut sum = 0;
	for game in games {
		let max = game.max_pulls();
		let power = max.red * max.green * max.blue;
		sum += power;
	}
	println!("Power of the sets: {}", sum);

}