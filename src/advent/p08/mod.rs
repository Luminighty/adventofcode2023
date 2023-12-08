use std::collections::HashMap;

use self::map::{Turn, NodeName, Node};

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 8: Haunted Wasteland";

mod error;
mod input;
mod map;

fn step(map: &HashMap<NodeName, Node>, current: &str, turn: &Turn) -> Option<String> {
	map.get(current).map(|n| n.next(turn))
}

pub fn solve_a() {
	let (turns, map) = input::get_input().unwrap();
	let mut current = String::from("AAA");
	let mut steps = 0;
	for turn in turns.iter().cycle() {
		steps += 1;
		if let Some(next) = step(&map, &current, turn) {
			current = next;
		} else {
			println!("Node '{}' not found", current);
			return;
		}
		if current == "ZZZ" {
			break;
		}
	}
	println!("Steps: {}", steps);
}



fn is_done(current: &Vec<NodeName>) -> bool {
	current.iter().all(|c| c.ends_with('Z'))
}

pub fn solve_b() {
	let (turns, map) = input::get_input().unwrap();
	let mut steps = 0;
	let mut current: Vec<NodeName> = map.iter()
		.filter(|(node, _)| node.ends_with('A'))
		.map(|(node, _)| node.to_owned())
		.collect();

	for turn in turns.iter().cycle() {
		steps += 1;
		for current in current.iter_mut() {
			if let Some(next) = step(&map, &current, turn) {
				*current = next;
			} else {
				println!("Node '{}' not found", current);
				return;
			}
		}

		if is_done(&current) {
			break;
		}
	}
	
	println!("Parallel Steps: {}", steps);
}
