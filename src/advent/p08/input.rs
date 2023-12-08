use std::collections::HashMap;

use super::{error::AdventError, map::{Turn, Node, NodeName}};

pub fn get_input() -> Result<(Vec<Turn>, HashMap<NodeName, Node>), AdventError> {
	let content = std::fs::read_to_string("inp/08.txt")
		.map_err(|_| AdventError::FileReadError)?;
	let mut parts = content.split("\n\n");
	let turns = parts.next().ok_or(AdventError::FileFormatError)?;
	let map = parts.next().ok_or(AdventError::FileFormatError)?;
	let turns = get_turns(turns)?;
	let map = get_map(map)?;
	Ok((turns, map))
}

fn get_turns(line: &str) -> Result<Vec<Turn>, AdventError> {
	line.chars()
		.map(parse_turn)
		.collect()
}

fn parse_turn(c: char) -> Result<Turn, AdventError> {
	match c {
		'R' => Ok(Turn::Right),
		'L' => Ok(Turn::Left),
		_ => Err(AdventError::UnknownTurn)
	}
}

fn get_map(map: &str) -> Result<HashMap<NodeName, Node>, AdventError> {
	map
		.lines()
		.map(parse_node)
		.collect()
}

fn parse_node(node: &str) -> Result<(NodeName, Node), AdventError> {
	let (name, turns) = node.split_once(" = ").ok_or(AdventError::NodeFormatError)?;
	let turns = turns.trim_start_matches('(').trim_end_matches(')');
	let (left, right) = turns.split_once(", ").ok_or(AdventError::NodeFormatError)?;
	Ok((name.to_owned(), Node::new(left.to_owned(), right.to_owned())))
}