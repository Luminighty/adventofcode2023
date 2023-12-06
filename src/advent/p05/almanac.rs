pub type Seed = u64;
pub struct Almanac {
	pub seeds: Vec<Seed>,
	maps: Vec<Map>
}

impl Almanac {
	pub fn new(seeds: Vec<Seed>, maps: Vec<Map>) -> Self {
		Self { seeds, maps }
	}

	pub fn map(&self, value: Seed) -> Seed {
		let mut value = value;
		for map in &self.maps {
			if let Some(new_value) = map.apply(value) {
				value = new_value
			}
		}
		value
	}
}

#[allow(dead_code)]
pub struct Map {
	name: String,
	ranges: Vec<Range>
}

impl Map {
	pub fn new(name: String, ranges: Vec<Range>) -> Self {
		Self { name, ranges }
	}

	pub fn apply(&self, value: Seed) -> Option<Seed> {
		for range in &self.ranges {
			if let Some(mapped_value) = range.map(value) {
				return Some(mapped_value);
			}
		}
		None
	}
}

pub struct Range {
	source: Seed,
	destination: Seed,
	length: Seed,
}

impl Range {
	pub fn new(source: Seed, destination: Seed, length: Seed) -> Self {
		Self { source, destination, length }
	}

	pub fn map(&self, value: Seed) -> Option<Seed> {
		if self.in_source(value) {
			Some(value - self.source + self.destination)
		} else {
			None
		}
	}

	fn in_source(&self, value: Seed) -> bool {
		self.source <= value && self.source + self.length > value
	}
}

