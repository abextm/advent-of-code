use std::collections::{HashMap, HashSet};
use crate::grid::{Grid, Ve};

#[aoc(part1=423)]
fn part1(input: &str, part1: bool) -> impl std::fmt::Debug {
	let input = Grid::from_char_grid(input);
	let mut antis = HashSet::<Ve<2>>::new();

	let mut nodes = HashMap::<u8, Vec<Ve<2>>>::new();
	for (pt, k) in input.filter_enumerate(|&v| v != b'.') {
		nodes.entry(*k).or_default().push(pt);
	}

	for (_, set) in nodes.iter() {
		for i in 0..set.len() {
			for j in (i+1)..set.len() {
				let delta = set[i] - set[j];
				let pt = set[i] + delta;
				if input.get(pt) != None {
					antis.insert(pt);
				}
				let pt = set[j] - delta;
				if input.get(pt) != None {
					antis.insert(pt);
				}
			}
		}
	}

	let mut dbg = input.cloned();
	for pt in antis.iter() {
		dbg[*pt] = b'#';
	}
	println!("{}", std::str::from_utf8(&dbg.array).unwrap());
	
	antis.len()
}

#[aoc(part2)]
fn part2(input: &str, part1: bool) -> impl std::fmt::Debug {
	let input = Grid::from_char_grid(input);
	let mut antis = HashSet::<Ve<2>>::new();

	let mut nodes = HashMap::<u8, Vec<Ve<2>>>::new();
	for (pt, k) in input.filter_enumerate(|&v| v != b'.') {
		nodes.entry(*k).or_default().push(pt);
	}

	for (_, set) in nodes.iter() {
		for i in 0..set.len() {
			for j in (i+1)..set.len() {
				let delta = set[i] - set[j];

				let mut pt = set[i];
				while let Some(_) = input.get(pt) {
					antis.insert(pt);
					pt = pt + delta;
				}

				let mut pt = set[j];
				while let Some(_) = input.get(pt) {
					antis.insert(pt);
					pt = pt - delta;
				}
			}
		}
	}

	let mut dbg = input.cloned();
	for pt in antis.iter() {
		dbg[*pt] = b'#';
	}
	println!("{}", std::str::from_utf8(&dbg.array).unwrap());

	antis.len()
}


#[aoc(part1=14, part2=34)]
const EX: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";