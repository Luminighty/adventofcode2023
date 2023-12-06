pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 5: If You Give A Seed A Fertilizer";

mod error;
mod input;
mod almanac;

pub fn solve_a() {
	let almanac = input::get_input().unwrap();
	
	let min = almanac.seeds.clone().into_iter()
		.map(|s| almanac.map(s))
		.min();
	println!("Lowest Location {:?}", min);
}

pub fn solve_b() {
	let almanac = input::get_input().unwrap();

	let mut min = None;
	for i in (0..almanac.seeds.len()).step_by(2) {
		let start = almanac.seeds[i];
		let length = almanac.seeds[i + 1];
		for seed in start..(start+length) {
			let location = almanac.map(seed);
			min = match min {
				Some(n) if n > location => Some(location),
				None => Some(location),
				_ => min,
			}
		}
	}

	println!("Lowest Seed Range Location: {:?}", min);
}
