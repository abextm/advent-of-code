use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::grid::Grid;

#[derive(Copy, Clone, Eq, PartialEq)]
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

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn cost(g: &Grid<u8>) -> Option<usize> {
	let mut dist = Grid::blank(g, usize::max_value());
	let mut heap = BinaryHeap::new();

	let start = (0, 0);
	let end = (g.width() - 1, g.height() - 1);

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

		for (x, y, &v) in g.adjacent4(pos.0, pos.1) {
			let next = State {
				cost: cost + v as usize,
				pos: (x, y),
			};

			if next.cost < dist[[x, y]] {
				heap.push(next);
				dist[next.pos] = next.cost;
			}
		}
	}

	None
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
	let g = Grid::from_number_grid(input);
	cost(&g).unwrap()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
	let ig = Grid::from_number_grid(input);
	let g = Grid::from_generator(&(ig.width() * 5, ig.height() * 5), |x, y| {
		let v = *ig.get_wrapped(x as isize, y as isize);
		let tile = (x / ig.width() + y / ig.height()) as u8;
		let v = v + tile;
		((v - 1) % 9) + 1
	});

	cost(&g).unwrap()
}

#[cfg(test)]
const EXAMPLE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

#[test]
fn test_p1() {
	assert_eq!(40, part1(EXAMPLE));
}

#[test]
fn test_p2() {
	assert_eq!(315, part2(EXAMPLE));
}