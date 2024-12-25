use crate::grid::Grid;

#[aoc(part1 = 3483)]
fn solve(input: &str) -> impl std::fmt::Debug {
	let mut keys = Vec::new();
	let mut locks = Vec::new();

	for grid in input.trim().split("\n\n") {
		let grid = Grid::from_char_grid(grid);
		let v = grid[[0, 0]];
		let mut heights = Vec::new();
		for x in 0..grid.shape()[0] {
			let mut height = 1;
			while Some(&v) == grid.get([x, height]) {
				height += 1;
			}

			height -= 1;
			heights.push(height);
		}

		match v {
			b'#' => &mut locks,
			b'.' => &mut keys,
			_ => panic!(),
		}.push(heights);
	}

	let mut n = 0;
	for lock in locks.iter() {
		for key in keys.iter() {
			if key.iter().zip(lock).all(|(k, l)| k >= l) {
				n += 1;
			}
		}
	}

	n
}

#[aoc(part1 = 3)]
const EX: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";