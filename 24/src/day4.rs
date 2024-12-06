use crate::grid;
use crate::grid::{Grid, Ve};

#[aoc(part1=2613)]
fn part1(input: &str) -> impl std::fmt::Debug {
	let g = Grid::from_char_grid(input);

	g.find(b'X')
		.map(|x_pt| {
			grid::adj8()
				.filter(|delta| b"XMAS".iter()
					.enumerate()
					.skip(1)
					.all(|(i, v)| Some(v) == g.get(x_pt + delta * Ve::from(i))))
				.count()
		})
		.sum::<usize>()
}


#[aoc(part2=1905)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let g = Grid::from_char_grid(input);

	g.find(b'A')
		.map(|pt| {
			let corners = [[-1, -1], [-1, 1], [1, 1], [1, -1]].map(|d| g.get(pt + Ve::from(d)).cloned());

			for off in 0..4 {
				if corners[(0 + off) & 3] == Some(b'M')
					&& corners[(1 + off) & 3] == Some(b'M')
					&& corners[(2 + off) & 3] == Some(b'S')
					&& corners[(3 + off) & 3] == Some(b'S') {
					return 1;
				}
			}

			return 0;
		})
		.sum::<i32>()
}

#[aoc(part1=18, part2=9)]
const EX: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";