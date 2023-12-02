pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 1: Trebuchet?!";

#[derive(Debug)]
enum AdventError {
	FileReadError,
}

fn get_input() -> Result<Vec<String>, AdventError> {
	let content = std::fs::read_to_string("inp/01.txt").map_err(|_| AdventError::FileReadError)?;
	Ok(content.split("\n")
			.map(|s| String::from(s))
			.collect()
	)
}

fn parse_char(c: char) -> Option<i32> {
	match c {
			'0' => Some(0),
			'1' => Some(1),
			'2' => Some(2),
			'3' => Some(3),
			'4' => Some(4),
			'5' => Some(5),
			'6' => Some(6),
			'7' => Some(7),
			'8' => Some(8),
			'9' => Some(9),
			_ => None,
	}
}

fn parse_word(s: &str) -> Option<i32> {
	let words = [
			("one", 1),
			("two", 2),
			("three", 3),
			("four", 4),
			("five", 5),
			("six", 6),
			("seven", 7),
			("eight", 8),
			("nine", 9),
	];
	for (word, value) in words {
			if s.ends_with(word) {
					return Some(value);
			}
	}
	return None;
}


fn calibrate(value: &String) -> Option<i32> {
	let mut first: Option<i32> = None;
	let mut last: Option<i32> = None;
	for c in value.chars() {
			let parsed = parse_char(c);
			first = first.or(parsed.clone());
			last = parsed.or(last);
	}
	first.zip(last).map(|(f, l)| f * 10 + l)
}


fn calibrate_b(value: &String) -> Option<i32> {
	let mut first: Option<i32> = None;
	let mut last: Option<i32> = None;
	let mut word: String = String::new();
	for c in value.chars() {
			word = word + &String::from(c);
			let word_parsed = parse_word(&word);
			let parsed = parse_char(c).or(word_parsed);
			first = first.or(parsed.clone());
			last = parsed.or(last);
	}
	first.zip(last).map(|(f, l)| f * 10 + l)
}

pub fn solve_b() {
	let input = get_input().unwrap();

	let mut sum = 0;
	for data in input {
			if let Some(calibrated) = calibrate_b(&data) {
					sum += calibrated;
			} else {
					println!("'{}' does not contain the calibrated value", &data);
		 }
	}
	println!("Result: {}", sum);

}


pub fn solve_a() {
	let input = get_input().unwrap();

	let mut sum = 0;
	for data in input {
			if let Some(calibrated) = calibrate(&data) {
					sum += calibrated;
			} else {
					println!("'{}' does not contain the calibrated value", &data);
		 }
	}
	println!("Result: {}", sum);

}