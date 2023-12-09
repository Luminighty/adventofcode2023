pub type Reading = i64;

pub struct Report {
	readings: Vec<Vec<Reading>>
}

#[derive(Debug)]
pub enum ReportError {
	MissingReading,
}

impl Report {
	pub fn new(initial: Vec<Reading>) -> Self {
		Self { readings: vec![initial] }
	}

	fn interpolate_step(&self) -> Result<Vec<Reading>, ReportError> {
		let last = self.readings.last().ok_or(ReportError::MissingReading)?;
		let mut interpolated = Vec::new();
		for i in 0..(last.len() - 1) {
			let left = last[i];
			let right = last[i + 1];
			interpolated.push(right - left);
		}
		Ok(interpolated)
	}

	fn is_interpolation_done(&self) -> bool {
		if let Some(readings) = self.readings.last() {
			readings.iter().all(|&r| r == 0)
		} else {
			false
		}
	}

	pub fn interpolate(&mut self) -> Result<(), ReportError> {
		while !self.is_interpolation_done() {
			self.readings.push(self.interpolate_step()?);
		}
		Ok(())
	}

	pub fn extrapolate(&mut self) -> Result<Reading, ReportError> {
		let mut value = 0;
		for i in (1..self.readings.len()).rev() {
			self.readings[i].push(value);
			let before = self.readings[i - 1].last().ok_or(ReportError::MissingReading)?;
			value = before + value;
		}
		self.readings[0].push(value);
		Ok(value)
	}

	pub fn extrapolate_back(&mut self) -> Result<Reading, ReportError> {
		let mut value = 0;
		for i in (1..self.readings.len()).rev() {
			self.readings[i].insert(0, value);
			let before = self.readings[i - 1].first().ok_or(ReportError::MissingReading)?;
			value = before - value;
		}
		self.readings[0].insert(0, value);
		Ok(value)
	}

	pub fn print(&self) {
		for reading in &self.readings {
			println!("{:?}", reading);
		}
		println!("");
	}
}