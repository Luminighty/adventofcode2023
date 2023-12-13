use super::map::Map;

#[derive(Debug)]
pub enum Reflection {
	Vertical(usize),
	Horizontal(usize),
}

pub fn find(map: &Map) -> Option<Reflection> {
	for x in 1..map.width(0) {
		if is_vertical_mirror(map, x) == 0 {
			return Some(Reflection::Vertical(x));
		}
	}
	for y in 1..map.height() {
		if is_horizontal_mirror(map, y) == 0 {
			return Some(Reflection::Horizontal(y));
		}
	}
	None
}

pub fn find_smudges(map: &Map) -> Option<Reflection> {
	for x in 1..map.width(0) {
		if is_vertical_mirror(map, x) == 2 {
			return Some(Reflection::Vertical(x));
		}
	}
	for y in 1..map.height() {
		if is_horizontal_mirror(map, y) == 2 {
			return Some(Reflection::Horizontal(y));
		}
	}
	None
}

fn is_vertical_mirror(map: &Map, mirror_x: usize) -> usize {
	let mut difference = 0;
	for y in 0..map.height() {
		for x in 0..map.width(y) {
			let tile = map.get(x, y);
			let mirrored = mirror(x, mirror_x);
			if mirrored < 0 { continue; }
			let mirrored = map.get(mirrored as usize, y);
			if mirrored.is_some() && tile != mirrored {
				difference += 1;
			}
		}
	}
	difference
}

fn is_horizontal_mirror(map: &Map, mirror_y: usize) -> usize {
	let mut difference = 0;
	for y in 0..map.height() {
		for x in 0..map.width(y) {
			let tile = map.get(x, y);
			let mirrored = mirror(y, mirror_y);
			if mirrored < 0 { continue; }
			let mirrored = map.get(x, mirrored as usize);
			if mirrored.is_some() && tile != mirrored {
				difference += 1;
			}
		}
	}
	difference
}

fn mirror(value: usize, mirror: usize) -> isize {
	let value = value as isize;
	let mirror = mirror as isize;
	mirror + (mirror - value) - 1
}
