pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 17: Clumsy Crucible";

mod error;
mod input;
mod heat;
mod path;

pub fn solve_a() {
	let map = input::get_input().unwrap();

	let target_y = map.height() - 1;
	let target_x = map.width(target_y) - 1;

	let result = path::find(
		map, 
		(0, 0), 
		(target_x as isize, target_y as isize)
	);
	println!("Minimum heatloss: {:?}", result);
}

pub fn solve_b() {

}
  