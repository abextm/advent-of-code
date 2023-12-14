use crate::grid::Grid;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn shift<const DIR: isize>(grid: &mut Grid<&mut [u8]>) {
	let (start, end) = if DIR == -1 {
		(grid.height() as isize - 1, 0)
	} else {
		(0, grid.height() as isize - 1)
	};
	for x in 0..grid.width() as isize {
		let mut y_blank = start;
		let mut y_search = start;
		loop {
			match grid[[x, y_search]] {
				b'O' => {
					grid[[x, y_search]] = b'.';
					grid[[x, y_blank]] = b'O';
					y_blank += DIR;
				},
				b'#' => {
					y_blank = y_search + DIR;
				},
				_ => {},
			}
			if y_search == end {
				break;
			}
			y_search += DIR;
		}
	}
}

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
	let mut grid = Grid::from_char_grid(input).owned_copy();

	shift::<1>(&mut grid.as_mut_ref());

	grid.filter_enumerate(|&x| x == b'O')
		.map(|(_x, y, _v)| grid.height() - y)
		.sum()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
	let mut grid = Grid::from_char_grid(input).owned_copy();

	let mut history = HashMap::<crate::grid::FastHash<1, u8>, usize>::new();

	let target_cycles = 1_000_000_000;
	let mut cycle = 0;

	while cycle < target_cycles {
		shift::<1>(&mut grid.as_mut_ref());
		shift::<1>(&mut grid.as_mut_ref().transposed());
		shift::<-1>(&mut grid.as_mut_ref());
		shift::<-1>(&mut grid.as_mut_ref().transposed());

		cycle += 1;
		match history.entry(crate::grid::FastHash(grid.clone())) {
			Entry::Occupied(v) => {
				let last_cycle = v.get();
				let delta = cycle - last_cycle;
				cycle += (target_cycles - cycle) / delta * delta;
			},
			Entry::Vacant(v) => {
				v.insert(cycle);
			},
		}
	}

	grid.filter_enumerate(|&x| x == b'O')
		.map(|(_x, y, _v)| grid.height() - y)
		.sum()
}

#[cfg(test)]
const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[test]
fn test_p1() {
	assert_eq!(136, part1(EXAMPLE));
}

#[test]
fn test_p2() {
	assert_eq!(64, part2(EXAMPLE));
}