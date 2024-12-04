use crate::grid;
use crate::grid::Grid;

#[aoc(part1=2613)]
fn solve(input: &str) -> impl std::fmt::Debug {
	let g = Grid::from_char_grid(input);

	g.filter_enumerate(|&x| x == b'X')
		.map(|(x, y, _v)| {
			grid::ADJ8.iter()
				.filter(|(dx, dy)| b"XMAS".iter()
					.enumerate()
					.skip(1)
					.all(|(i, v)| Some(v) == g.get(x as isize + dx * i as isize, y as isize + dy * i as isize)))
				.count()
		})
		.sum::<usize>()
}


#[aoc(part2)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let g = Grid::from_char_grid(input);

	g.filter_enumerate(|&x| x == b'A')
		.map(|(x, y, _v)| {
			let corners = [(-1, -1), (-1, 1), (1, 1), (1, -1)].map(|(dx, dy)| g.get(x as isize + dx, y as isize + dy).cloned());

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