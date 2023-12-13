pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 12: Hot Springs";

mod error;
mod input;
mod parts;

pub fn solve_a() {
	let records = input::get_input().unwrap();
	let mut sum = 0;
	for record in records {
		let arrangements = parts::arrangements(&record);
		if parts::DEBUG { println!("{:?} => {}", record, arrangements); }
		sum += arrangements;
	}
	println!("Sum of possible arrangements: {}", sum);
}

pub fn solve_b() {

}
