use std::num::ParseIntError;

use super::{error::AdventError, almanac::{Map, Almanac, Range, Seed}};

pub fn get_input() -> Result<Almanac, AdventError> {
	let content = std::fs::read_to_string("inp/05.txt").map_err(|_| AdventError::FileReadError)?;
	let (seeds, maps) = content.as_str().split_once("\n\n").ok_or(AdventError::FileFormatError)?;
	let seeds = parse_seeds(seeds)?;
	let maps = parse_maps(maps)?;
	Ok(Almanac::new(seeds, maps))
}

fn parse_seeds(seeds: &str) -> Result<Vec<Seed>, ParseIntError> {
	seeds
		.replace("seeds: ", "")
		.as_str()
		.split(' ')
		.map(str::parse)
		.collect()
}

fn parse_maps(maps: &str) -> Result<Vec<Map>, AdventError> {
	maps
		.split("\n\n")
		.map(parse_map)
		.collect()
}

fn parse_map(map: &str) -> Result<Map, AdventError> {
	let (name, ranges) = map.split_once('\n')
		.ok_or(AdventError::MapFormatError(map.to_owned()))?;
	let ranges = ranges
		.split('\n')
		.map(parse_range)
		.collect::<Result<Vec<Range>, AdventError>>()?;
	Ok(Map::new(name.to_owned(), ranges))
}

fn parse_range(range: &str) -> Result<Range, AdventError> {
	let mut range_iter = range.splitn(3, ' ');
	let dest = range_iter.next().ok_or(AdventError::RangeFormatError(range.to_owned()))?;
	let source = range_iter.next().ok_or(AdventError::RangeFormatError(range.to_owned()))?;
	let length = range_iter.next().ok_or(AdventError::RangeFormatError(range.to_owned()))?;
	Ok(Range::new(
		source.parse()?, 
		dest.parse()?,
		 length.parse()?
	))
}