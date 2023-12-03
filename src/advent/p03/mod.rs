pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 3: Gear Ratios";

mod error;
mod input;
mod engine;

fn has_adjacent_symbol(x: isize, y: isize, symbols: &Vec<engine::SymbolPart>) -> bool {
	for symbol in symbols {
		if symbol.is_adjacent(x, y) {
			return true
		}
	}
	false
}

fn sum_part_numbers(numbers: Vec<engine::NumberPart>, symbols: Vec<engine::SymbolPart>) -> u32 {
	let mut sum = 0;
	for number in numbers {
		for dx in 0..number.width {
			if has_adjacent_symbol(number.x + dx as isize, number.y, &symbols) {
				sum += number.number;
				break;
			}
		}
	}
	sum
}

fn get_gear_ratio(x: isize, y: isize, numbers: &Vec<engine::NumberPart>) -> u32 {
	let mut adjacent = Vec::new();
	for number in numbers {
		for dx in 0..number.width {
			let delta_x = ((number.x + dx as isize) - x).abs();
			let delta_y = (number.y - y).abs();
			let delta = delta_x.max(delta_y);
			if delta <= 1 {
				adjacent.push(number.number);
				break;
			}
		}
	}
	match (adjacent.get(0), adjacent.get(1), adjacent.get(2)) {
		(Some(a), Some(b), None) => a * b,
		_ => 0,
	}
}

pub fn solve_a() {
	let engine = input::get_input().unwrap();
	let number_parts = engine::get_number_parts(&engine);
	let symbol_parts = engine::get_symbol_parts(&engine);
	println!("{}", sum_part_numbers(number_parts, symbol_parts));
}

pub fn solve_b() {
	let engine = input::get_input().unwrap();
	let number_parts = engine::get_number_parts(&engine);
	let symbol_parts = engine::get_symbol_parts(&engine);

	let mut sum = 0;
	for symbol in symbol_parts {
		if symbol.symbol != '*' { continue; }
		sum += get_gear_ratio(symbol.x, symbol.y, &number_parts);
	}
	println!("{}", sum);
}
