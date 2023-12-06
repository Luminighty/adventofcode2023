pub type Unit = u64;

#[derive(Debug)]
pub struct Race {
	pub distance: Unit,
	pub time: Unit,
}

impl Race {
	pub fn new(time: Unit, distance: Unit) -> Self {
		Self { time, distance }
	}

	pub fn total_distance(&self, hold_time: Unit) -> Unit {
		return hold_time * (self.time - hold_time);
	}

	pub fn merge(races: Vec<Race>) -> Self {
		let dist = races
			.iter()
			.flat_map(|r| digits(r.distance))
			.collect::<Vec<Unit>>();
		let time = races
			.iter()
			.flat_map(|r| digits(r.time))
			.collect::<Vec<Unit>>();
		Self::new(
			digits_to_number(time), 
			digits_to_number(dist)
		)	
	}

}

pub fn digits_to_number(digits: Vec<Unit>) -> Unit {
	let mut n = 0;
	for d in digits {
		n = n * 10 + d;
	}
	n
}

pub fn digits(number: Unit) -> Vec<Unit> {
	let mut digit = Vec::new();
	let mut number = number;
	while number > 0 {
		digit.push(number % 10);
		number /= 10;
	}
	digit.reverse();
	digit
}