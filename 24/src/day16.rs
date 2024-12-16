use std::collections::{HashSet, VecDeque};
use crate::dijkstra::{dijkstra, State};
use crate::grid::{Grid, Ve};

const DIRS: [Ve<2>; 4] = [
	Ve([0, -1]),
	Ve([1, 0]),
	Ve([0, 1]),
	Ve([-1, 0]),
];

#[aoc(part1 = 98484, part2=531)]
fn solve(input: &str, part1: bool) -> impl std::fmt::Debug {
	let grid = Grid::from_char_grid(input);
	let start = grid.find(b'S').next().unwrap();
	let end = grid.find(b'E').next().unwrap();

	let (cost, dist) = dijkstra(start.cat1::<3>(1), Grid::new(grid.shape().cat1(4), Default::default()), |v| v.part() == end, |pt| {
		let grid = &grid;
		let dir = pt[2];
		let pt = pt.part();
		[(0, 1), (1, 1001), (-1, 1001)].into_iter().filter_map(move |(dd, cost)| {
			let nd = (dir + dd) & 3;
			let npt = pt + DIRS[nd as usize];
			match grid[npt] {
				b'#' | b'S' => None,
				b'.' | b'E' => Some((npt.cat1(nd), cost)),
				_ => panic!(),
			}
		})
	}).unwrap();

	if part1 {
		cost
	} else {
		let mut edge = VecDeque::new();
		for i in 0..4 {
			let npt = end.cat1(i);
			if let Some(v) = dist[npt] {
				edge.push_back(State{
					index: npt,
					cost: v.0,
				});
			}
		}

		let mut visited = HashSet::new();

		while let Some(state) = edge.pop_front() {
			visited.insert(state.index.part::<2>());
			let dir = state.index[2];
			let pt = state.index.part();
			for (dd, dcost) in [(0, 1), (1, 1001), (-1, 1001)] {
				let nd = (dir + dd) & 3;
				let npt = pt - DIRS[dir as usize];
				if let Some((cost, _)) = dist[npt.cat1(nd)] {
					if cost + dcost == state.cost {
						edge.push_front(State {
							cost,
							index: npt.cat1(nd),
						});
					}
				}
			}
		}

		visited.len()
	}
}

#[aoc(part1 = 7036, part2 = 45)]
const EX: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
