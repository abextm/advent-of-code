use crate::grid::Grid;

#[aoc(day11, part1)]
fn part1(input: &str) -> isize {
	solve(input, 2)
}
#[aoc(day11, part2)]
fn part2(input: &str) -> isize {
	solve(input, 1000000)
}

fn solve(input: &str, adj: isize) -> isize {
	let g = Grid::from_char_grid(input);

	let mut map = [
		vec![false; g.width()],
		vec![false; g.height()],
	];

	let planets = g.filter_enumerate(|&x| x == b'#')
		.map(|(x, y, _v)| {
			map[0][x] = true;
			map[1][y] = true;
			(x, y)
		})
		.collect::<Vec<_>>();

	let map = map.map(|a| {
		let mut x = 0isize;
		a.into_iter().map(|v| {
			let o_x = x;
			x += if !v { adj } else { 1 };
			o_x
		}).collect::<Vec<_>>()
	});

	let planets = planets.into_iter()
		.map(|(x, y)| (map[0][x], map[1][y]))
		.collect::<Vec<_>>();

	planets.iter()
		.enumerate()
		.flat_map(|(i, a)|
			planets[i + 1..].iter()
				.map(move |b| (a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()))
		.sum()
}

#[test]
fn test_p1() {
	assert_eq!(8410, solve("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", 100));
}