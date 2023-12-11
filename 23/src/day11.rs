use crate::grid::Grid;

fn rowcol_map<const is_rows: bool>(g: &Grid<&[u8], u8>, adj: usize) -> Vec<usize> {
	let (dim, odim) = if is_rows {
		(g.height(), g.width())
	} else {
		(g.width(), g.height())
	};

	let mut out = Vec::with_capacity(dim);
	let mut adj_x = 0;
	for x in 0..dim {
		out.push(adj_x);
		adj_x += 1;
		if !(0..odim).any(|y|{
			let val = if is_rows { g[[y, x]] } else { g [[x, y]] };
			val != b'.'
		}) {
			adj_x += adj - 1;
		}
	}
	out
}

#[aoc(day11, part1)]
fn part1(input: &str) -> isize {
	solve(input, 2)
}
#[aoc(day11, part2)]
fn part2(input: &str) -> isize {
	solve(input, 1000000)
}

fn solve(input: &str, adj: usize) -> isize {
	let g = Grid::from_char_grid(input);

	let map = (
		rowcol_map::<false>(&g, adj),
		rowcol_map::<true>(&g, adj),
	);

	let planets = g.filter_enumerate(|&x| x == b'#')
		.map(|(x, y, _v)| (map.0[x], map.1[y]))
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
#...#.....", 99));
}