use self::galaxy::Expansion;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 11: Cosmic Expansion";

mod error;
mod input;
mod galaxy;

fn distance(a: (isize, isize), b: (isize, isize)) -> isize {
	(a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn weighted_distance(
	a: (isize, isize), 
	b: (isize, isize), 
	(columns, rows): &Expansion, 
	weight: isize
) -> isize {
	let min_x = a.0.min(b.0);
	let max_x = a.0.max(b.0);
	let min_y = a.1.min(b.1);
	let max_y = a.1.max(b.1);

	let mut column_gaps = 0;
	let mut row_gaps = 0;
	for &c in columns {
		if c > min_x && c < max_x {
			column_gaps += weight - 1;
		}
	}
	for &r in rows {
		if r > min_y && r < max_y {
			row_gaps += weight - 1;
		}
	}
	(a.0 - b.0).abs() + (a.1 - b.1).abs() + column_gaps + row_gaps
}

pub fn solve_a() {
	let mut image = input::get_input().unwrap();
	image.expand();
	let galaxies = image.galaxies();
	let mut sum = 0;
	for i in 0..galaxies.len() {
		for j in i..galaxies.len() {
			sum += distance(galaxies[i], galaxies[j]);
		}
	}
	println!("Sum of shortest path: {}", sum);
}

pub fn solve_b() {
	let image = input::get_input().unwrap();
	let expansion = image.expand_meta();
	
	let galaxies = image.galaxies();
	let mut sum = 0;
	for i in 0..galaxies.len() {
		for j in i..galaxies.len() {
			sum += weighted_distance(galaxies[i], galaxies[j], &expansion, 1_000_000);
		}
	}
	println!("Sum of shortest path: {}", sum);
}
