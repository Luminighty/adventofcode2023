#[derive(Debug)]
pub enum Turn {
	Left,
	Right
}

pub type NodeName = String;

#[derive(Debug)]
pub struct Node {
	pub left: NodeName,
	pub right: NodeName,
}

impl Node {
	pub fn new(left: NodeName, right: NodeName) -> Self {
		Self { left, right }
	}

	pub fn next(&self, turn: &Turn) -> NodeName {
		match turn {
			Turn::Left => self.left.to_owned(),
			Turn::Right => self.right.to_owned(),
		}
	}
}