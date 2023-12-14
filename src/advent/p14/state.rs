use std::collections::HashSet;
use super::dish::Pos;

pub struct State {
	state: HashSet<Pos>
}

impl State {
	pub fn new(state: &Vec<Pos>) -> Self {
		Self { state: state.clone().into_iter().collect() }
	}
}

impl PartialEq for State {
	fn eq(&self, other: &Self) -> bool {
		self.state == other.state
	}
}

impl Eq for State {}