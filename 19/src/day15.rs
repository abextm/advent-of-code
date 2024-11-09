use crate::vm;
use std::collections::{hash_map, BinaryHeap, HashMap};

#[derive(Clone, Eq, PartialEq)]
struct State {
	cost: usize,
	success: bool,
	position: (isize, isize),
}

impl Ord for State {
	fn cmp(&self, other: &State) -> std::cmp::Ordering {
		other
			.cost
			.cmp(&self.cost)
			.then_with(|| self.position.cmp(&other.position))
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

mod status {
	pub const HIT_WALL: i64 = 0;
	pub const OK: i64 = 1;
	pub const OXYGEN: i64 = 2;
}

struct Explorer {
	root: vm::State<std::iter::Empty<i64>>,
	last: HashMap<(isize, isize), (isize, isize)>,
	dist: HashMap<(isize, isize), usize>,
	heap: BinaryHeap<State>,
}

impl Explorer {
	fn new(root: vm::State<std::iter::Empty<i64>>, start: (isize, isize)) -> Explorer {
		let mut e = Explorer {
			root,
			last: HashMap::new(),
			dist: HashMap::new(),
			heap: BinaryHeap::new(),
		};
		e.dist.insert(start, 0);
		e.heap.push(State {
			cost: 0,
			success: false,
			position: start,
		});
		e
	}

	fn solve<R, F: Fn(&State) -> Option<R>>(&mut self, f: F) -> Option<R> {
		while let Some(state) = self.heap.pop() {
			if let Some(v) = f(&state) {
				return Some(v);
			}
			if let Some(previous_cost) = self.dist.get(&state.position) {
				if *previous_cost < state.cost {
					continue;
				}
			}
			let instrs = self.instrs_for_position(state.position);
			let vm = self.advance_vm_to(&instrs);

			for (ins, dir) in &[(1, (-1, 0)), (2, (1, 0)), (3, (0, 1)), (4, (0, -1))] {
				match vm.clone().with_input([*ins].iter().cloned()).next() {
					Some(Ok(status::HIT_WALL)) => (),
					Some(Ok(s)) => {
						let new_state = State {
							position: (state.position.0 + dir.0, state.position.1 + dir.1),
							success: s == status::OXYGEN,
							cost: state.cost + 1,
						};
						let d = match self.dist.entry(new_state.position) {
							hash_map::Entry::Vacant(e) => e.insert(std::usize::MAX),
							hash_map::Entry::Occupied(e) => e.into_mut(),
						};
						if *d > new_state.cost {
							*d = new_state.cost;
							self.last.insert(new_state.position, state.position);
							self.heap.push(new_state);
						}
					}
					v => panic!("{:?}", v),
				}
			}
		}

		None
	}

	fn instrs_for_position(&self, position: (isize, isize)) -> Vec<i64> {
		let mut instrs = Vec::new();
		let mut pos = position;
		while let Some(&prev) = self.last.get(&pos) {
			instrs.push(match (pos.0 - prev.0, pos.1 - prev.1) {
				(-1, 0) => 1,
				(1, 0) => 2,
				(0, 1) => 3,
				(0, -1) => 4,
				v => panic!("{:?}", v),
			});
			pos = prev;
		}
		instrs.reverse();
		instrs
	}

	fn advance_vm_to(&self, ins: &Vec<i64>) -> vm::State<std::iter::Empty<i64>> {
		let mut vm = self.root.clone().with_input(ins.iter().cloned());
		loop {
			match vm.next() {
				Some(Err(vm::EvalError::EndOfInput)) => break,
				Some(Ok(status::HIT_WALL)) => panic!("wall?"),
				Some(Ok(_)) => (),
				v => panic!("{:?}", v),
			};
		}
		vm.without_input()
	}
}

#[aoc(day15, part1)]
fn day15_part1(input: &str) -> usize {
	day15(input, false)
}

#[aoc(day15, part2)]
fn day15_part2(input: &str) -> usize {
	day15(input, true)
}

fn day15(input: &str, part2: bool) -> usize {
	let root = vm::new_from_str(input).unwrap();

	let mut find_o2 = Explorer::new(root, (0, 0));
	let o2 = find_o2
		.solve(|state| {
			if state.success {
				Some(state.clone())
			} else {
				None
			}
		})
		.unwrap();

	if !part2 {
		return o2.cost;
	}

	let instrs = find_o2.instrs_for_position(o2.position);
	let o2root = find_o2.advance_vm_to(&instrs);

	let mut ex = Explorer::new(o2root, o2.position);
	ex.solve::<(), _>(|_| None);

	*ex.dist.values().max().unwrap()
}
