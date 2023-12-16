pub fn hash(word: &str) -> u8 {
	let mut current: usize = 0;
	for c in word.chars() {
		let ascii = c as u8;
		current += ascii as usize;
		current *= 17;
		current %= 256;
	}
	current as u8
}

pub type Item = u32;

pub enum Operation {
	Remove(String),
	Add(String, Item),
}

// Holiday ASCII String Helper Manual Arrangement Procedure
pub struct HashMap {
	boxes: Vec<Vec<(String, Item)>>,
}

impl HashMap {
	pub fn new() -> Self {
		Self { boxes: vec![Vec::new(); 256] }
	}

	fn get(&mut self, key: &str) -> &mut Vec<(String, Item)> {
		&mut self.boxes[hash(&key) as usize]
	}

	pub fn add(&mut self, key: String, value: Item) {
		let b = self.get(&key);
		for i in 0..b.len() {
			if b[i].0 != key { continue; }
			b[i].1 = value;
			return;
		}
		b.push((key, value))
	}

	pub fn remove(&mut self, key: String) {
		let b = self.get(&key);

		for i in 0..b.len() {
			if b[i].0 != key { continue; }
			b.remove(i);
			return;
		}
	}

	pub fn focusing_power(&self) -> usize {
		let mut sum = 0;
		for i in 0..self.boxes.len() {
			let b = &self.boxes[i];
			for (slot, (_, lens)) in b.iter().enumerate() {
				let box_number = i + 1;
				let slot = slot + 1;
				sum += box_number * slot * *lens as usize;
			}
		}
		sum
	}
}

