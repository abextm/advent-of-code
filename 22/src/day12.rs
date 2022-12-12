use crate::grid::Grid;
use std::collections::BinaryHeap;
use std::cmp::Ordering;


#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
struct State {
	cost: usize,
	pos: (usize, usize),
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		other.cost.cmp(&self.cost)
			.then_with(|| self.pos.0.cmp(&other.pos.0))
			.then_with(|| self.pos.1.cmp(&other.pos.1))
	}
}

fn cost(g: &Grid<u8>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
	let mut dist = Grid::blank(g, usize::max_value());
	let mut heap = BinaryHeap::new();

	heap.push(State {
		cost: 0,
		pos: start,
	});
	while let Some(State{cost, pos}) = heap.pop() {
		if pos == end {
			return Some(cost);
		}

		if cost > dist[pos] {
			continue;
		}

		let h = g[pos] as isize;
		for (x, y, &v) in g.adjacent4(pos.0, pos.1) {
			if h + 1 >= v as isize {
				let next = State {
					cost: cost + 1,
					pos: (x, y),
				};
	
				if next.cost < dist[[x, y]] {
					heap.push(next);
					dist[next.pos] = next.cost;
				}
			}
		}
	}

	None
}

#[aoc(day12, part1)]
fn day12_part1(input: &str) -> usize {
	let mut grid = Grid::from_str_with_mapper(input, |&x| x as u8);
	let start = grid.find(&b'S').unwrap();
	let end = grid.find(&b'E').unwrap();

	grid[start] = b'a';
	grid[end] = b'z';

	cost(&grid, start, end).unwrap()
}
#[aoc(day12, part2)]
fn day12_part2(input: &str) -> usize {
	let mut grid = Grid::from_str_with_mapper(input, |&x| x as u8);
	let start = grid.find(&b'S').unwrap();
	let end = grid.find(&b'E').unwrap();

	grid[start] = b'a';
	grid[end] = b'z';

	grid.iter()
		.filter(|&(_, _, &v)| v == b'a')
		.filter_map(|(x, y, _)| cost(&grid, (x, y), end))
		.min()
		.unwrap()
}