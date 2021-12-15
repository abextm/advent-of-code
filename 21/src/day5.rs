use crate::grid::Grid;
use std::cmp::{Ord, Ordering, min, max};

#[aoc(day5, part1)]
fn day5_part1(input: &str) -> usize {
	solve(input, false)
}

#[aoc(day5, part2)]
fn day5_part2(input: &str) -> usize {
	solve(input, true)
}

fn solve(input: &str, part2: bool) -> usize {
	let lines: Vec<(usize, usize, usize, usize)> = input.trim().lines()
		.map(|x| {
			let mut p = x.split(" -> ")
				.flat_map(|x| x.split(",")
					.map(|x| x.parse().unwrap()));
			(p.next().unwrap(), p.next().unwrap(), p.next().unwrap(), p.next().unwrap())
		}).collect();

	let maxb = (
		lines.iter().map(|x| max(x.0, x.2)).max().unwrap() + 1,
		lines.iter().map(|x| max(x.1, x.3)).max().unwrap() + 1,
	);

	let mut grid = Grid::blank(&maxb, 0);

	for (x0, y0, x1, y1) in lines {
		let dx = -ccmp(&x0, &x1);
		let dy = -ccmp(&y0, &y1);
		if !part2 && dx != 0 && dy != 0 {
			continue;
		}
		let d = if dy == 0 {
			max(x0, x1) - min(x0, x1)
		} else {
			max(y0, y1) - min(y0, y1)
		} as isize;
		for i in 0..=d {
			let x = x0.wrapping_add((dx*i) as usize);
			let y = y0.wrapping_add((dy*i) as usize);
			grid[[x, y]] += 1;
		}
	}

	grid.values_iter().filter(|&&x| x > 1).count()
}

fn ccmp<T: Ord>(a: &T, b: &T) -> isize {
	match a.cmp(b) {
		Ordering::Less => -1,
		Ordering::Equal => 0,
		Ordering::Greater => 1,
	}
}

#[test]
fn test_day5() {
	let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

	assert_eq!(day5_part1(input), 5);
	assert_eq!(day5_part2(input), 12);
}
