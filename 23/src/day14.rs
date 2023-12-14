use crate::grid::Grid;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn cycle_n(grid: &mut Grid<&mut [u8]>) {
	for y in 1..grid.height() {
		for x in 0..grid.width() {
			if grid[[x, y]] == b'O' {
				let mut y2 = y as isize;
				loop {
					y2 -= 1;
					if y2 < 0 || grid[[x, y2 as usize]] != b'.' {
						break;
					}
				}
				let y2 = (y2 + 1) as usize;
				if y2 != y {
					grid[[x, y]] = b'.';
					grid[[x, y2]] = b'O';
				}
			}
		}
	}
}
fn cycle_s(grid: &mut Grid<&mut [u8]>) {
	for y in (0..(grid.height() - 1)).rev() {
		for x in 0..grid.width() {
			if grid[[x, y]] == b'O' {
				let mut y2 = y;
				loop {
					y2 += 1;
					if y2 >= grid.height() || grid[[x, y2 as usize]] != b'.' {
						break;
					}
				}
				let y2 = y2 - 1;
				if y2 != y {
					grid[[x, y]] = b'.';
					grid[[x, y2]] = b'O';
				}
			}
		}
	}
}

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
	let mut grid = Grid::from_char_grid(input).owned_copy();

	cycle_n(&mut grid.as_mut_ref());

	grid.filter_enumerate(|&x| x == b'O')
		.map(|(_x, y, _v)| grid.height() - y)
		.sum()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
	let mut grid = Grid::from_char_grid(input).owned_copy();

	let mut history = HashMap::<Grid<Vec<u8>>, usize>::new();

	let target_cycles = 1_000_000_000;
	let mut cycle = 0;

	while cycle < target_cycles {
		cycle_n(&mut grid.as_mut_ref());
		cycle_n(&mut grid.as_mut_ref().transposed());
		cycle_s(&mut grid.as_mut_ref());
		cycle_s(&mut grid.as_mut_ref().transposed());

		cycle += 1;
		match history.entry(grid.clone()) {
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