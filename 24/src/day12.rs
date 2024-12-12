use std::collections::VecDeque;
use crate::grid::{adj4, Grid, Ve};

#[aoc(part1 = 1375574, part2 = 830566)]
fn solve(input: &str, part1: bool) -> impl std::fmt::Debug {
	let mut input = Grid::from_char_grid(input).map(|_pt, v| *v as u16);
	let mut edge = VecDeque::new();
	let mut cost = 0;
	let mut index = 255;
	for pt in input.points() {
		let char = input[pt];
		if char > 255 {
			continue;
		}

		edge.clear();
		edge.push_back(pt);

		index += 1;
		input[pt] = index;

		let mut area = 0;
		let mut perim = 0;

		while let Some(pt) = edge.pop_front() {
			area += 1;
			for delta in adj4() {
				let npt = pt + delta;
				let v = input.get(npt);
				if v == Some(&char) {
					input[npt] = index;
					edge.push_back(npt);
				} else if v != Some(&index) {
					if part1 {
						perim += 1;
					} else {
						let right_pt = pt + Ve([-delta[1], delta[0]]);
						let right_forward_pt = right_pt + delta;
						match input.get(right_pt) {
							Some(&v) if v == char || v == index => {
								if matches!(input.get(right_forward_pt), Some(&v) if v == char || v == index) {
									perim += 1
								}
							}
							_ => perim += 1,
						}
					}
				}
			}
		}

		cost += perim * area;
	}

	cost
}

#[aoc(part1 = 1930, part2 = 1206)]
const EX: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";