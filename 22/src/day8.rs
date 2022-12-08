use crate::grid::Grid;

#[aoc(day8, part1)]
fn day8_part1(input: &str) -> usize {
	let grid = Grid::from_number_grid(input);
	grid.iter().filter(|&(x, y, &v)| {
		(0..x).map(|ix| grid[[ix, y]] + 1).max().unwrap_or(0) <= v
		|| ((x+1)..grid.width()).map(|ix| grid[[ix, y]] + 1).max().unwrap_or(0) <= v
		|| (0..y).map(|iy| grid[[x, iy]] + 1).max().unwrap_or(0) <= v
		|| ((y+1)..grid.height()).map(|iy| grid[[x, iy]] + 1).max().unwrap_or(0) <= v
	}).count()
}
#[aoc(day8, part2)]
fn day8_part2(input: &str) -> usize {
	let grid = Grid::from_number_grid(input);
	grid.iter().map(|(x, y, &v)| {
		((x - (0..x).rev().find(|&ix| grid[[ix, y]] >= v).unwrap_or(0)))
		* ((((x+1)..(grid.width())).find(|&ix| grid[[ix, y]] >= v).unwrap_or(grid.width() - 1)) - x)
		* ((y - (0..y).rev().find(|&iy| grid[[x, iy]] >= v).unwrap_or(0)))
		* ((((y+1)..(grid.height()))).find(|&iy| grid[[x, iy]] >= v).unwrap_or(grid.height() - 1) - y)
	}).max().unwrap_or(0)
}

#[cfg(test)]
const EXAMPLE_1: &str = "30373
25512
65332
33549
35390";

#[test]
fn test() {
	assert_eq!(day8_part1(EXAMPLE_1), 21);
	assert_eq!(day8_part2(EXAMPLE_1), 8);
}