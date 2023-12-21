use std::collections::VecDeque;

use crate::grid::Grid;

type Cell = u8;
const ROCK: Cell = Cell::MAX;
const UNVISITED: Cell = Cell::MAX - 1;
const START: Cell = Cell::MAX - 2;

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
	let mut start = (0usize, 0usize);
	let mut g = Grid::from_char_grid(input).map(|x, y, v| match v {
		b'#' => ROCK,
		b'.' => UNVISITED,
		b'S' => {
			start = (x, y);
			START
		},
		_ => panic!(),
	});

	let mut edge = Vec::new();
	let mut next_edge = Vec::new();
	next_edge.push(start);

	let mut visited_plots = 0;
	for steps in 0..64 {
		edge.clear();
		std::mem::swap(&mut edge, &mut next_edge);
		visited_plots = 0;
		for pt in edge.iter() {
			for n in crate::grid::adjacent4_points(&g, pt.0, pt.1) {
				let cell = &mut g[n];
				if *cell == ROCK || *cell == steps {
					continue;
				} else {
					*cell = steps;
					next_edge.push(n);
					visited_plots += 1;
				}
			}
		}
	}

	visited_plots
}

#[test]
fn test_p1() {
	assert_eq!(116, part1("...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."));
}