use crate::grid::{adj4, Grid, Ve};

#[aoc(part1=1343, part2=982891)]
fn solve(input: &str, part1: bool) -> impl std::fmt::Debug {
	solve0(input, 100, if part1 { 2 } else { 20 })
}

fn solve0(input: &str, min_save: i32, cheat_len: i32) -> i32 {
	let adj_list: Vec<Ve<2>> = (-cheat_len..=cheat_len).flat_map(|x| (-cheat_len..=cheat_len).map(move |y| {Ve::from([x, y])}))
		.filter(|pt| (pt[0].abs() + pt[1].abs()) as i32 <= cheat_len)
		.collect();

	let g = Grid::from_char_grid(input);

	let start = g.find(b'S').next().unwrap();

	let mut min = Grid::new(g.shape(), i32::MAX);
	min[start] = 0;

	let mut todo = Vec::new();
	todo.push((start, 0));

	while let Some((pt, cost)) = todo.pop() {
		let cost = cost + 1;
		for d in adj4() {
			let npt = pt + d;
			if let Some(&v) = g.get(npt) {
				if v == b'.' || v == b'E' {
					if min[npt] > cost {
						min[npt] = cost;
						if v != b'E' {
							todo.push((npt, cost));
						}
					}
				}
			}
		}
	}

	let mut cheats = 0;
	for (pt, &v) in min.iter() {
		if v != i32::MAX {
			for &d in adj_list.iter() {
				if let Some(iv) = min.get(pt + d) {
					let savings = v - iv;
					if savings >= min_save + (d[0].abs() + d[1].abs()) as i32 {
						cheats += 1;
					}
				}
			}
		}
	}

	cheats
}


#[test]
fn part1_example() {
	assert_eq!(solve0(EX, 20, 2), 5);
	assert_eq!(solve0(EX, 2, 2), 14 + 14 + 2 + 4 + 2 + 3 + 5);
}

#[cfg(test)]
const EX: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";