use crate::advent::p13::reflection::Reflection;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 13: Point of Incidence";

mod error;
mod input;
mod map;
mod reflection;

pub fn solve_a() {
	let maps = input::get_input().unwrap();

	let mut sum = 0;
	for map in maps {
		let value = match reflection::find(&map) {
			Some(Reflection::Vertical(n)) => n,
			Some(Reflection::Horizontal(n)) => 100 * n,
			_ => {
				println!("Reflection not found!");
				println!("{:?}", map);
				return;
			}
		};
		sum += value;
	}
	println!("Mirror note: {}", sum);
}

pub fn solve_b() {
	let maps = input::get_input().unwrap();

	let mut sum = 0;
	for map in maps {
		let value = match reflection::find_smudges(&map) {
			Some(Reflection::Vertical(n)) => n,
			Some(Reflection::Horizontal(n)) => 100 * n,
			_ => {
				println!("Reflection not found!");
				println!("{:?}", map);
				return;
			}
		};
		sum += value;
	}
	println!("Mirror note: {}", sum);

}
