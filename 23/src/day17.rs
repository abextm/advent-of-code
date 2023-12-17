use crate::grid::Grid;
use std::{collections::BinaryHeap, cmp::Reverse};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct State {
	heat_loss: usize,
	pos: (isize, isize),
	direction: (isize, isize),
	distance_in_direction: usize,
}

fn cost(g: &Grid<Vec<u8>>, start: (isize, isize), end: (isize, isize), p2: bool) -> Option<usize> {
	let mut dist = Grid::blank(g, [[[usize::max_value(); 10]; 2]; 2]);
	let mut heap = BinaryHeap::new();
	let mut next_states = Vec::new();

	heap.push(Reverse(State {
		heat_loss: 0,
		pos: start,
		direction: (1, 0),
		distance_in_direction: 0,
	}));
	heap.push(Reverse(State {
		heat_loss: 0,
		pos: start,
		direction: (0, 1),
		distance_in_direction: 0,
	}));
	while let Some(Reverse(state)) = heap.pop() {
		if state.pos == end && (!p2 || state.distance_in_direction >= 3) {
			return Some(state.heat_loss);
		}

		if state.heat_loss > dist[state.pos][(state.direction.0 == 1) as usize][(state.direction.1 == 1) as usize][state.distance_in_direction] {
			continue;
		}

		next_states.clear();

		if state.distance_in_direction < if p2 { 9 } else { 2 } {
			next_states.push(State {
				distance_in_direction: state.distance_in_direction + 1,
				..state
			});
		}
		if !p2 || state.distance_in_direction >= 3 {
			next_states.push(State {
				distance_in_direction: 0,
				direction: (state.direction.1, state.direction.0),
				..state
			});
			next_states.push(State {
				distance_in_direction: 0,
				direction: (-state.direction.1, -state.direction.0),
				..state
			});
		}

		for state in next_states.iter_mut() {
			state.pos.0 += state.direction.0;
			state.pos.1 += state.direction.1;
			if let Some(tile_loss) = g.get(state.pos.0, state.pos.1) {
				state.heat_loss += *tile_loss as usize;
	
				if state.heat_loss < dist[state.pos][(state.direction.0 == 1) as usize][(state.direction.1 == 1) as usize][state.distance_in_direction] {
					heap.push(Reverse(*state));
					dist[state.pos][(state.direction.0 == 1) as usize][(state.direction.1 == 1) as usize][state.distance_in_direction] = state.heat_loss;
				}
			}
		}
	}

	None
}

#[aoc(day17, part1)]
fn part1(input: &str) -> usize {
	let grid = Grid::from_number_grid(input);
	cost(&grid, (0, 0), (grid.width() as isize - 1, grid.height() as isize - 1), false).unwrap()
}

#[aoc(day17, part2)]
fn part2(input: &str) -> usize {
	let grid = Grid::from_number_grid(input);
	cost(&grid, (0, 0), (grid.width() as isize - 1, grid.height() as isize - 1), true).unwrap()
}


#[cfg(test)]
const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

#[test]
fn test_p1() {
	assert_eq!(102, part1(EXAMPLE))
}

#[test]
fn test_p2() {
	assert_eq!(94, part2(EXAMPLE))
}