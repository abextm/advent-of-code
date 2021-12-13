use std::cmp::max;
use crate::grid::Grid;
#[aoc(day13, part1)]
fn day13_part1(input: &str) -> usize {
	let mut input = input.split("\n\n");
	let (mut size, mut grid) = {
		let input: Vec<_> = input.next().unwrap().lines().map(|x| {
			let mut it = x.split(",").map(|x|x.parse::<usize>().unwrap());
			(it.next().unwrap(), it.next().unwrap())
		}).collect();
		let mut size = input.iter().fold((0, 0), |acc, v| (max(acc.0, v.0), max(acc.1, v.1)));
		size.0 += 1;
		size.1 += 1;
		let mut grid = Grid::blank(size.0, size.1, false);
		for (x, y) in input {
			grid.set(x, y, true);
		}
		(size, grid)
	};

	for line in input.next().unwrap().lines() {
		let mut lit = line.split("=");
		let axis = lit.next().unwrap();
		let axis = axis.chars().rev().next().unwrap();
		let point = lit.next().unwrap().parse::<usize>().unwrap();

		let relsz = match axis {
			'x' => &mut size.0,
			'y' => &mut size.1,
			_ => panic!(),
		};

		*relsz = point;
		let mut out = Grid::blank(size.0, size.1, false);

		for y in 0..size.1 as isize {
			for x in 0..size.0 as isize {
				out.set(x as usize, y as usize, grid.get_safe(x, y).unwrap()
				| if axis == 'y' { grid.get_wrapped(x, -y - 1) } else {grid.get_wrapped(-x -1, y)});
			}
		}

		grid = out;
		break
	}

	grid.iter().map(|(_, _, &v)| v as usize).sum::<usize>()
}

#[cfg(test)]
const EXAMPLE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

#[test]
fn test_p1() {
	assert_eq!(17, day13_part1(EXAMPLE));
	panic!();
}

#[aoc(day13, part2)]
fn day13_part2(input: &str) -> usize {
	let mut input = input.split("\n\n");
	let (mut size, mut grid) = {
		let input: Vec<_> = input.next().unwrap().lines().map(|x| {
			let mut it = x.split(",").map(|x|x.parse::<usize>().unwrap());
			(it.next().unwrap(), it.next().unwrap())
		}).collect();
		let mut size = input.iter().fold((0, 0), |acc, v| (max(acc.0, v.0), max(acc.1, v.1)));
		size.0 += 1;
		size.1 += 1;
		size.1 |= 1; // even folds break, not the right fix
		let mut grid = Grid::blank(size.0, size.1, false);
		for (x, y) in input {
			grid.set(x, y, true);
		}
		(size, grid)
	};

	for line in input.next().unwrap().lines() {
		let mut lit = line.split("=");
		let axis = lit.next().unwrap();
		let axis = axis.chars().rev().next().unwrap();
		let point = lit.next().unwrap().parse::<usize>().unwrap();

		let relsz = match axis {
			'x' => &mut size.0,
			'y' => &mut size.1,
			_ => panic!(),
		};

		*relsz = point;
		let mut out = Grid::blank(size.0, size.1, false);

		for y in 0..size.1 as isize {
			for x in 0..size.0 as isize {
				out.set(x as usize, y as usize, grid.get_safe(x, y).unwrap()
				| if axis == 'y' { grid.get_wrapped(x, -y - 1) } else {grid.get_wrapped(-x -1, y)});
			}
		}

		grid = out;
	}

	grid.print_b();

	0
}
