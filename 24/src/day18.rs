use crate::dijkstra::{dijkstra, State};
use crate::grid::{adj4, Grid, Ve};

#[aoc(part1=308)]
fn part1(input: &str) -> impl std::fmt::Debug {
	solve(input, 1024).unwrap()
}

#[aoc(part2="\"46,28\"")]
fn part2_2(input: &str) -> impl std::fmt::Debug {
	let mut g = Grid::new([71, 71], u16::MAX);
	for (i, line) in input.trim().lines().enumerate() {
		let pt = Ve::<2>(line.split(",").map(|n| n.parse().unwrap()).next_chunk().unwrap());
		g[pt] = i as u16;
	}

	let mut dist = Grid::new(g.shape(), false);
	let goal_tester = |&pt: &Ve<2>| {pt[0] == 0 || pt[1] == 70};

	let mut heap = std::collections::BinaryHeap::new();
	for start in (1..=70).map(|x| Ve([x, 0])).chain((1..70).map(|y| Ve([70, y]))) {
		let v = g[start];
		if v != u16::MAX {
			let cost = v as usize;
			heap.push(State {
				cost,
				index: start,
			});
		}
	}

	while let Some(state) = heap.pop() {
		if goal_tester(&state.index) {
			return input.lines().nth(state.cost).unwrap().to_owned();
		}

		if dist[state.index] {
			continue;
		}
		dist[state.index] = true;

		let pt = state.index;
		let g = &g;
		for (index, cost) in adj4().filter_map(move |d| {
			if let Some(&v) = g.get(pt + d) {
				if v != u16::MAX {
					let v = v as usize;
					return Some((pt + d, state.cost.max(v)))
				}
			}
			return None
		}) {
			if !dist[index] {
				heap.push(State {
					index: index.clone(),
					cost,
				});
			}
		}
	}

	panic!()
}

#[allow(unused)]
fn part2_1(input: &str) -> impl std::fmt::Debug {
	let n = input.lines().count();
	let mut low = 0;
	let mut high = n - 1;
	while low <= high {
		let mid = (low + high) / 2;
		if let Some(_) = solve(input, mid + 1) {
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

