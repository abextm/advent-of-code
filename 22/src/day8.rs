use crate::grid::Grid;

#[aoc(day8, part1)]
fn day8_part1(input: &str) -> usize {
	let grid = Grid::from_number_grid(input);
	grid.iter()
		.filter(|&(x, y, &v)| grid.directions_from(x, y)
			.any(|mut it| it
				.all(|(_, _, &iv)| iv < v)))
		.count()
}
#[aoc(day8, part2)]
fn day8_part2(input: &str) -> usize {
	let grid = Grid::from_number_grid(input);
	grid.iter()
		.map(|(x, y, &v)| grid.directions_from(x, y)
			.map(|it| {
				let mut i = 0;
				for (_, _, &iv) in it {
					i += 1;
					if iv >= v {
						break;
					}
				}
				i
			})
		.reduce(|a, b| a * b)
		.unwrap())
	.max()
	.unwrap_or(0)
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