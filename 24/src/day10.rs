use std::collections::VecDeque;
use crate::grid::{adj4, Grid};

#[aoc(part1 = 461, part2 = 875)]
fn solve(input: &str, part1: bool) -> impl std::fmt::Debug {
	let input = Grid::from_char_grid(input);

	let mut visited = Grid::new(input.shape(), 0u16);
	let mut edge = VecDeque::new();
	let mut trailheads = 0;
	let mut paths = 0;
	for pt in input.find(b'0') {
		edge.push_back((pt, b'0', None));
		visited.array.fill(0);
		while let Some((pt, v, last_pt)) = edge.pop_front() {
			let nv = v + 1;
			let ways_to_point = if let Some(last_pt) = last_pt {
				visited[last_pt]
			} else {
				1
			};
			for delta in adj4() {
				let new_pt = pt + delta;
				if input.get(new_pt) == Some(&nv) {
					let first_visit = visited[new_pt] == 0;
					visited[new_pt] += ways_to_point;
					if nv == b'9' {
						paths += ways_to_point
					}
					if first_visit {
						if nv == b'9' {
							trailheads += 1
						} else {
							edge.push_back((new_pt, nv, Some(pt)));
						}
					}
				}
			}
		}
	}

	if part1 {
		trailheads
	} else {
		paths
	}
}


#[aoc(part1 = 36, part2 = 81)]
const EX: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";