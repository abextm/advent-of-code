use crate::grid::{Grid, points, adjacent8_points};

fn inc(grid: &mut Grid<u8>, x: usize, y: usize) {
	let v = &mut grid[[x, y]];
	if *v < 10 {
		*v += 1;
		if *v == 10 {
			for (x, y) in adjacent8_points(grid, x, y) {
				inc(grid, x, y);
			}
		}
	}
}

fn advance(grid: &mut Grid<u8>) -> usize {
	for (x, y) in points(grid) {
		inc(grid, x, y);
	}
	let mut flashes = 0;
	for (x, y) in points(grid) {
		let v = &mut grid[[x, y]];
		if *v >= 10 {
			flashes += 1;
			*v = 0;
		}
	}
	flashes
}

#[aoc(day11, part1)]
fn day11_part1(input: &str) -> usize {
	let mut grid = Grid::from_number_grid(input);
	let mut flashes = 0;
	for _ in 0..100 {
		flashes += advance(&mut grid);
	}
	flashes
}
#[aoc(day11, part2)]
fn day11_part2(input: &str) -> usize {
	let mut grid = Grid::from_number_grid(input);
	for i in 1.. {
		let flashes = advance(&mut grid);
		if flashes == grid.width() * grid.height() {
			return i;
		}
	}
	panic!();
}

#[cfg(test)]
const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

#[test]
fn test_p1() {
	assert_eq!(1656, day11_part1(EXAMPLE));
}
#[test]
fn test_p2() {
	assert_eq!(195, day11_part2(EXAMPLE));
}