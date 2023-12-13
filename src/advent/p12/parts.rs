use std::{fmt::Debug, collections::VecDeque};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Condition {
	Operational,
	Damaged,
	Unknown
}

impl Debug for Condition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Condition::Operational => '.',
			Condition::Damaged => '#',
			Condition::Unknown => '?',
		})
  }
}

#[derive(Debug)]
pub struct Record {
	pub record: Vec<Condition>,
	pub groups: VecDeque<usize>,
}

impl Record {
	pub fn new(record: Vec<Condition>, groups: VecDeque<usize>) -> Self {
		Self { record, groups }
	}
}

pub const DEBUG: bool = false;

pub fn arrangements(record: &Record) -> usize {
	read_known(record, 0, 0).unwrap_or_default()
}

fn read_known(record: &Record, mut index: usize, mut group: usize) -> Option<usize> {
	if DEBUG { println!("{}read_known {} {}", " ".repeat(index), index, group); }
	let mut group_size = 0;
	while record.record.len() > index {
		match record.record[index] {
			Condition::Damaged => { group_size += 1;},
			Condition::Operational if Some(&group_size) == record.groups.get(group) => {
				group_size = 0;
				group += 1;
			},
			Condition::Operational if group_size > 0 => { 
				return None; 
			},
			Condition::Operational => {}
			// It can only be operational, so we reset the group
			Condition::Unknown if Some(&group_size) == record.groups.get(group) => {
				group_size = 0;
			},
			// It can only be damaged or wrong
			Condition::Unknown if group_size > 0 => {
				group_size += 1;
			},
			Condition::Unknown => { return read_unknown(record, index, group); }
		}
		index += 1;
	}
	if Some(&group_size) == record.groups.get(group) {
		group += 1;
		group_size = 0;
	}
	if DEBUG { println!("end: {} {} = {}", index, group, if group_size == 0 && record.groups.len() == group { 1 } else { 0 }); }
	Some(if group_size == 0 && record.groups.len() == group { 1 } else { 0 })
}

fn as_damaged(record: &Record, index: usize, group: usize) -> Option<usize> {
	if DEBUG { println!("{}as_damaged {} {}", " ".repeat(index), index, group); }
	if group >= record.groups.len() {
		return None;
	}
	let group_size = record.groups[group];
	if index + group_size >= record.record.len() {
		return None;
	}
	for i in index..(index + group_size) {
		if record.record[i] == Condition::Operational {
			return None;
		}
	}
	// Case: ???# 3
	if record.record.get(index + group_size) == Some(&Condition::Damaged) {
		return None;
	}
	if index + group_size == record.record.len() && group + 1 == record.groups.len() {
		Some(1)
	} else {
		read_known(record, index + group_size + 1, group + 1)
	}
}

fn as_operational(record: &Record, index: usize, group: usize) -> Option<usize> {
	if DEBUG { println!("{}as_operational {} {}", " ".repeat(index), index, group); }
	if index + 1 == record.record.len() && group + 1 == record.groups.len() {
		Some(1)
	} else {
		read_known(record, index + 1, group)
	}
}

fn read_unknown(record: &Record, index: usize, group: usize) -> Option<usize> {
	let damaged = as_damaged(record, index, group);
	let operational: Option<usize> = as_operational(record, index, group);
	if damaged.is_none() && operational.is_none() {
		return None;
	}
	Some(damaged.unwrap_or_default() + operational.unwrap_or_default())
}
