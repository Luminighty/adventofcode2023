#[derive(Debug)]
pub struct Card {
	pub id: u32,
	pub winning: Vec<u32>,
	pub actual: Vec<u32>
}

impl Card {
	pub fn new(id: u32, winning: Vec<u32>, actual: Vec<u32>) -> Self {
		Self { id, winning, actual }
	}
}