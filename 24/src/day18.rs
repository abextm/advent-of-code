use crate::dijkstra::dijkstra;
use crate::grid::{adj4, Grid, Ve};

#[aoc(part1=308)]
fn part1(input: &str) -> impl std::fmt::Debug {
	solve(input, 1024).unwrap()
}

#[aoc(part2="\"46,28\"")]
fn part2(input: &str) -> impl std::fmt::Debug {
	let n = input.lines().count();
	let mut low = 0;
	let mut high = n - 1;
	while low <= high {
		let mid = (low + high) / 2;
		if let Some(v) = solve(input, mid + 1) {
			low = mid + 1;
		} else {
			high = mid - 1;
		}
	}

	let mid = (low + high) / 2;
	input.lines().nth(mid + 1).unwrap().to_owned()
}

fn solve(input: &str, limit: usize) -> Option<usize> {
	let mut grid = Grid::new([71, 71], false);

	for line in input.lines().take(limit) {
		let pt = Ve::<2>(line.split(",").map(|n| n.parse().unwrap()).next_chunk().unwrap());
		grid[pt] = true;
	}

	let grid = &grid;
	dijkstra(Ve::zero(), Grid::new(grid.shape(), Default::default()), |&pt| pt == grid.shape() - Ve::from(1isize), |pt| {
		adj4().filter_map(move |d| {
			let npt = pt + d;
			if Some(false) == grid.get(npt).cloned() {
				Some((npt, 1))
			} else {
				None
			}
		})
	}).map(|v| v.0)
}

