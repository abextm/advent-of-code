use std::collections::HashMap;
use crate::grid::Grid;

pub fn dijkstra<Map, Index, Iter, IterGenerator, GoalTester>(
	start: Index,
	mut dist: Map,
	goal_tester: GoalTester,
	iter_generator: IterGenerator,
) -> Option<usize>
where
	Index: Eq + Clone,
	GoalTester: Fn(&Index) -> bool,
	Iter: Iterator<Item=(Index, usize)>,
	IterGenerator: Fn(Index) -> Iter,
	Map: StateMap<Index, Option<Index>>,
{
	let mut heap = std::collections::BinaryHeap::new();
	dist.insert_if_better(&start, (0, None));
	heap.push(State {
		cost: 0,
		index: start,
	});

	while let Some(state) = heap.pop() {
		if goal_tester(&state.index) {
			return Some(state.cost);
		}

		if dist.is_worse(&state.index, state.cost) {
			continue;
		}

		for (index, cost) in iter_generator(state.index.clone()) {
			let cost = state.cost + cost;

			if dist.insert_if_better(&index, (cost, Some(state.index.clone()))) {
				heap.push(State {
					index: index.clone(),
					cost,
				});
			}
		}
	}

	None
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<Index: Eq> {
	cost: usize,
	index: Index,
}

impl<Index: Eq> Ord for State<Index> {
	fn cmp(&self, other: &State<Index>) -> std::cmp::Ordering {
		other.cost.cmp(&self.cost)
	}
}

impl<Index: Eq> PartialOrd for State<Index> {
	fn partial_cmp(&self, other: &State<Index>) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

pub trait StateMap<K, V>
where
	K: Clone,
{
	fn insert_if_better(&mut self, key: &K, value: (usize, V)) -> bool;
	fn is_worse(&self, key: &K, value: usize) -> bool;
	fn get(&self, key: &K) -> Option<&(usize, V)>;
}

impl<K, V> StateMap<K, V> for HashMap<K, (usize, V)>
where
	K: Eq + std::hash::Hash + Clone,
{
	fn insert_if_better(&mut self, key: &K, value: (usize, V)) -> bool {
		let entry = self.entry(key.clone());
		match entry {
			std::collections::hash_map::Entry::Vacant(v) => {
				v.insert(value);
				true
			}
			std::collections::hash_map::Entry::Occupied(mut v) => {
				if v.get().0 <= value.0 {
					false
				} else {
					v.insert(value);
					true
				}
			}
		}
	}

	fn is_worse(&self, key: &K, cost: usize) -> bool {
		match self.get(key) {
			None => false,
			Some(v) => v.0 < cost,
		}
	}

	fn get(&self, key: &K) -> Option<&(usize, V)> {
		self.get(key)
	}
}

impl<V> StateMap<usize, V> for Vec<Option<(usize, V)>> {
	fn insert_if_better(&mut self, key: &usize, value: (usize, V)) -> bool {
		let val = self.get_mut(*key).expect("undersized map");
		match *val {
			None => {
				*val = Some(value);
				true
			}
			Some((cost, _)) => {
				if cost <= value.0 {
					false
				} else {
					*val = Some(value);
					true
				}
			}
		}
	}

	fn is_worse(&self, key: &usize, cost: usize) -> bool {
		match (**self).get(*key).expect("undersized map") {
			None => false,
			Some(v) => v.0 < cost,
		}
	}

	fn get(&self, key: &usize) -> Option<&(usize, V)> {
		(**self).get(*key).expect("undersized map").as_ref()
	}
}

impl<V> StateMap<(usize, usize), V> for Grid<Vec<Option<(usize, V)>>> {
	fn insert_if_better(&mut self, key: &(usize, usize), value: (usize, V)) -> bool {
		if let Some(v) = &self[*key] {
			if value.0 <= v.0 {
				return false
			}
		}
		self[*key] = Some(value);
		true
	}

	fn is_worse(&self, key: &(usize, usize), value: usize) -> bool {
		match &self[*key] {
			None => false,
			Some(v) => v.0 < value,
		}
	}

	fn get(&self, key: &(usize, usize)) -> Option<&(usize, V)> {
		self.get_unchecked(key.0, key.1).as_ref()
	}
}