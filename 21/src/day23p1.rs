use std::{cmp::Ordering, collections::HashMap, collections::BinaryHeap};

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
struct RoomState {
	rooms: [[u8; 2]; 4],
	hallway: [u8; 7],
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
	cost: usize,
	timestamp: usize,
	state: RoomState,
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		other.cost.cmp(&self.cost)
			.then_with(|| self.timestamp.cmp(&other.timestamp))
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn swap(from: &RoomState, color: u8, hw_index: usize, rm: usize, rm_index: usize, into: &mut Vec<State>) {
	let mut state = *from;
	std::mem::swap(&mut state.hallway[hw_index], &mut state.rooms[rm][rm_index]);
	let from = HALLWAY_POS[hw_index];
	let to = RM_POS[rm][rm_index];
	let dist = (from.0-to.0).abs() + (from.1-to.1).abs();
	let cost = dist as usize * match color {
		b'A' => 1,
		b'B' => 10,
		b'C' => 100,
		b'D' => 1000,
		_ => panic!(),
	};

	let l = from.0.min(to.0);
	let u = from.0.max(to.0);

	if HALLWAY_POS.iter().enumerate().filter(|(_, p)| p.0 > l && p.0 < u).all(|(i, _)| state.hallway[i] == NONE) {
		into.push(State {
			timestamp: 0,
			cost,
			state,
		});
	}
}
fn color_room(color: u8) -> usize {
	match color {
		b'A' => 0,
		b'B' => 1,
		b'C' => 2,
		b'D' => 3,
		_ => panic!(),
	}
}

const HALLWAY_POS: [(isize, isize); 7] = [
	(0, 0),
	(1, 0),
	(3, 0),
	(5, 0),
	(7, 0),
	(9, 0),
	(10, 0),
];
const RM_POS: [[(isize, isize); 2]; 4] = [
	[(2, 1), (2, 2)],
	[(4, 1), (4, 2)],
	[(6, 1), (6, 2)],
	[(8, 1), (8, 2)],
];

const NONE: u8 = b'.';

fn possible_moves(from: &RoomState, into: &mut Vec<State>) {
	for (hw_index, &a) in from.hallway.iter().enumerate() {
		if a == NONE {
			continue;
		}

		let rm = color_room(a);
		if !from.rooms[rm].iter().all(|&x| x == NONE || x == a) {
			continue
		}
		
		let rm_index = if from.rooms[rm][1] == NONE {1} else {0};

		swap(from, a, hw_index, rm, rm_index, into);
	}
	for rm in 0..4 {
		if let Some(rm_index) = from.rooms[rm].iter().position(|&x| x != NONE) {
			let a = from.rooms[rm][rm_index];
			for (hw_index, _) in from.hallway.iter().enumerate().filter(|&x| *x.1 == NONE) {
				swap(from, a, hw_index, rm, rm_index, into);
			}
		}
	}
}

fn cost(from: &RoomState, to: &RoomState) -> Option<usize> {
	let mut dist = HashMap::new();
	let mut prev = HashMap::new();
	let mut heap = BinaryHeap::new();

	let mut timestamp = 0;
	heap.push(State {
		cost: 0,
		timestamp,
		state: *from,
	});

	let mut into = Vec::new();
	while let Some(State{cost, timestamp: _, state}) = heap.pop() {
		if state == *to {
			let mut v = *to;
			loop {
				println!("{:?}", v);
				if let Some(pv) = prev.get(&v) {
					v = *pv;
				} else {
					break;
				}
			}
			return Some(cost);
		}

		if cost > dist.get(&state).cloned().unwrap_or(usize::MAX) {
			continue;
		}

		timestamp += 1;
		into.clear();
		possible_moves(&state, &mut into);
		while let Some(mut next) = into.pop() {
			next.cost += cost;
			next.timestamp = timestamp;
			if next.cost < dist.get(&next.state).cloned().unwrap_or(usize::MAX) {
				dist.insert(next.state.clone(), next.cost);
				prev.insert(next.state.clone(), state.clone());
				heap.push(next.clone());
			}
		}
	}

	None
}

const TARGET: RoomState = RoomState {
	rooms: [
		[b'A'; 2],
		[b'B'; 2],
		[b'C'; 2],
		[b'D'; 2],
	],
	hallway: [NONE; 7],
};

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
	let start = RoomState{
		rooms: [
			[b'C', b'B'],
			[b'D', b'A'],
			[b'A', b'D'],
			[b'B', b'C'],
		],
		hallway: [NONE; 7],
	};

	cost(&start, &TARGET).expect("no path")
}

#[test]
fn test() {
	let start = RoomState{
		rooms: [
			[b'B', b'A'],
			[b'C', b'D'],
			[b'B', b'C'],
			[b'D', b'A'],
		],
		hallway: [NONE; 7],
	};

	assert_eq!(cost(&start, &TARGET), Some(12521));
}