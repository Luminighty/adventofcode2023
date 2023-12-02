#[derive(Debug)]
pub struct Pull {
	pub red: u32,
	pub green: u32,
	pub blue: u32
}

impl Pull {
	pub fn new() -> Self {
		Self { red: 0, green: 0, blue: 0 }
	}

	pub fn pull_green(self, amount: u32) -> Self {
		Self { red: self.red, green: self.green + amount, blue: self.blue }
	}

	pub fn pull_blue(self, amount: u32) -> Self {
		Self { red: self.red, green: self.green, blue: self.blue + amount }
	}
	
	pub fn pull_red(self, amount: u32) -> Self {
		Self { red: self.red + amount, green: self.green, blue: self.blue }
	}

	pub fn max(self, other: &Self) -> Self {
		Self { 
			red: self.red.max(other.red),
			green: self.green.max(other.green),
			blue: self.blue.max(other.blue),
		}
	}

}

#[derive(Debug)]
pub struct Game {
	pub id: u32,
	pulls: Vec<Pull>
}

impl Game {

	pub fn new(id: u32, pulls: Vec<Pull>) -> Self {
		Self { id, pulls }
	}

	pub fn max_pulls(&self) -> Pull {
		let mut max_pull = Pull::new();

		for pull in &self.pulls {
			max_pull = max_pull.max(pull);
		}

		max_pull
	}

}
