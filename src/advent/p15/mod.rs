use self::hash::Operation;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 15: Lens Library";

mod error;
mod input;
mod hash;

pub fn solve_a() {
	let words = input::get_input().unwrap();

	let sum = words.iter()
		.map(|s| hash::hash(&s))
		.map(|a| a as usize)
		.reduce(|a, b| a + b);
	println!("Hashed sum: {:?}", sum);
}

pub fn solve_b() {
	let steps = input::get_steps().unwrap();

	let mut hashmap = hash::HashMap::new();

	for step in steps {
		match step {
			Operation::Add(label, lens) => { hashmap.add(label, lens) },
			Operation::Remove(label) => { hashmap.remove(label) },
		}
	}

	println!("Focusing Power: {}", hashmap.focusing_power());
}
