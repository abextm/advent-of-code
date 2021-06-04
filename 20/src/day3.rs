use crate::grid::Grid;

fn count(grid: &Grid, dx: usize, dy: usize) -> usize {
	let mut x = 0;
	let mut y = 0;
	let mut count = 0;
	while y < grid.height {
		if grid.get_wrapped_x(x, y) == '#' as u8 {
			count += 1;
		}
		x += dx;
		y += dy;
	}
	count
}

#[aoc(day3, part1)]
fn day3_part1(input: &str) -> usize {
	let grid = Grid::new(input);
	count(&grid, 3, 1)
}

#[aoc(day3, part2)]
fn day3_part2(input: &str) -> usize {
	let grid = Grid::new(input);
	count(&grid, 1, 1)
		* count(&grid, 3, 1)
		* count(&grid, 5, 1)
		* count(&grid, 7, 1)
		* count(&grid, 1, 2)
}
