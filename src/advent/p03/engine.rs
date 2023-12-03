pub type Engine = Vec<Vec<Part>>;

#[derive(Debug, PartialEq, Eq)]
pub enum Part {
	Digit(u32),
	Symbol(char),
	Space,
}

impl From<char> for Part {
	fn from(value: char) -> Self {
			match value {
				'.' => Self::Space,
				'0' => Self::Digit(0),
				'1' => Self::Digit(1),
				'2' => Self::Digit(2),
				'3' => Self::Digit(3),
				'4' => Self::Digit(4),
				'5' => Self::Digit(5),
				'6' => Self::Digit(6),
				'7' => Self::Digit(7),
				'8' => Self::Digit(8),
				'9' => Self::Digit(9),
				x => Self::Symbol(x),
			}
	}
}

#[derive(Debug)]
pub struct NumberPart {
	pub x: isize,
	pub y: isize,
	pub width: usize,
	pub number: u32,
}

impl NumberPart {
	pub fn new(x: usize, y: usize, width: usize, number: u32) -> Self {
		Self { x: x as isize, y: y as isize, width, number }
	}
}

#[derive(Debug)]
pub struct SymbolPart {
	pub x: isize,
	pub y: isize,
	pub symbol: char,
}

impl SymbolPart {
	pub fn new(x: usize, y: usize, symbol: char) -> Self {
		Self { x: x as isize, y: y as isize, symbol }
	}

	pub fn is_adjacent(&self, x: isize, y: isize) -> bool {
		isize::max((self.x - x).abs(), (self.y - y).abs()) <= 1
	}
}

pub fn get_number_parts(engine: &Engine) -> Vec<NumberPart> {
	let mut numbers = Vec::new();
	for (y, line) in engine.iter().enumerate() {
		let mut number = 0;
		let mut start_x = 0;
		let mut number_started = false;
		for (x, part) in line.iter().enumerate() {
			match part {
				Part::Digit(n) => { 
					if !number_started { start_x = x; }
					number_started = true;
					number = number * 10 + n 
				},
				_ => {
					if !number_started {
						continue;
					}
					numbers.push(NumberPart::new(start_x, y, x - start_x, number));
					number_started = false;
					number = 0;
				}
			}
		}
		if number_started {
			numbers.push(NumberPart::new(start_x, y, line.len() - start_x, number));
		}
	}
	numbers
}

pub fn get_symbol_parts(engine: &Engine) -> Vec<SymbolPart> {
	let mut symbols = Vec::new();
	for (y, line) in engine.iter().enumerate() {
		for (x, part) in line.iter().enumerate() {
			if let Part::Symbol(c) = part {
				symbols.push(SymbolPart::new(x, y, *c));
			}
		}
	}
	symbols
}