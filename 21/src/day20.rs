use crate::grid::{Grid, points};

pub const ADJ9: [(isize, isize); 9] = [
	(-1, -1),
	(0, -1),
	(1, -1),
	(-1, 0),
	(0, 0),
	(1, 0),
	(-1, 1),
	(0, 1),
	(1, 1),
];
#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
	solve(input, 2)
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
	solve(input, 50)
}

fn solve(input: &str, iterations: usize) -> usize {
	let (lut, mut grid) = {
		let mut it = input.trim().split("\n\n");
		let lut = it.next().unwrap().bytes().map(|x| x == b'#').collect::<Vec<bool>>();
		let grid = Grid::from_str_with_mapper(it.next().unwrap(), |x| *x == b'#');
		(lut, grid)
	};

	let outer_flag = lut[0];
	let mut default = outer_flag;
	for _ in 0..iterations {
		default ^= outer_flag;
		let mut next_grid = Grid::blank(&(grid.width() + 2, grid.height() + 2), false);
		for (x, y) in points(&next_grid) {
			let index = ADJ9.iter()
				.map(|(dx, dy)| grid.get(x as isize + dx - 1, y as isize + dy - 1).cloned().unwrap_or(default))
				.fold(0usize, |acc, v| acc << 1 | (v as usize));
			next_grid[[x, y]] = lut[index];
		}
		grid = next_grid;
	}

	grid.values_iter().filter(|&&x| x ).count()
}

#[cfg(test)]
const EXAMPLE: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

#[test]
fn test() {
	assert_eq!(part1(EXAMPLE), 35);
}