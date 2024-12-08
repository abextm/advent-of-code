use std::collections::HashMap;
use crate::grid::{Grid, Ve};

#[aoc(part1=423, part2=1287)]
fn solve<const PART1: bool>(input: &str) -> impl std::fmt::Debug {
	let input = Grid::from_char_grid(input);

	let mut nodes = HashMap::<u8, Vec<Ve<2>>>::new();
	for (pt, k) in input.filter_enumerate(|&v| v != b'.') {
		nodes.entry(*k).or_default().push(pt);
	}

	let mut antis = Grid::new(input.shape(), false);
	let mut num_antis = 0usize;

	for (_, set) in nodes.iter() {
		for i in 0..set.len() {
			for j in (i + 1)..set.len() {
				let delta = set[i] - set[j];
				let pairs = [(set[i], delta), (set[j], -delta)];
				for (start, delta) in pairs {
					for dist in if PART1 { 1..2 } else { 0usize..99 } {
						if let Some(v) = antis.get_mut(start + delta * Ve::from(dist)) {
							num_antis += (!*v) as usize;
							*v = true;
						} else {
							break;
						}
					}
				}
			}
		}
	}

	num_antis
}

#[aoc(part1 = 14, part2 = 34)]
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