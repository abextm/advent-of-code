use std::collections::HashMap;
use std::cmp::Ord;
use regex::Regex;
use crate::grid::Grid;

type ValveID = usize;
type ValveState = usize;

#[derive(Clone, Default, PartialEq, Eq, Debug)]
struct Valve {
	flow_rate: usize,
	children: Vec<ValveID>,
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Vec<Valve> {
	let mut valves: Vec<Valve> = Vec::new();
	let mut names: HashMap<String, ValveID> = HashMap::new();
	names.insert("AA".into(), 0);
	let re = Regex::new(r"^Valve ([^ ]+) has flow rate=([-0-9]+); tunnels? leads? to valves? (.*)$").unwrap();
	for line in input.lines() {
		let [name, flow_rate, children] = crate::re_captures(&re, line).unwrap();
		let len = names.len();
		let name = *names.entry(name.to_string()).or_insert(len);
		let flow_rate = flow_rate.parse::<usize>().unwrap();
		let children = children.split(", ").map(|x| {
			let len = names.len();
			*names.entry(x.to_string()).or_insert(len)
		}).collect();
		while valves.len() <= name {
			valves.push(Valve::default());
		}
		valves[name] = Valve { flow_rate, children};
	}

	valves
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct State<const P: usize> {
	closed_valves: ValveState,
	location: [ValveID; P],
	time_remaining: [i32; P],
}

struct Max<T: Ord> (T);

impl<T: Ord> Max<T> {
	fn push(&mut self, v: T) {
		if self.0 < v {
			self.0 = v;
		}
	}
}

fn visit<const P: usize>(valves: &[Valve], edges: &Grid<i32>, state: State<P>) -> usize {
	let mut v = Max(0);

	let max_rem = *state.time_remaining.iter().max().unwrap();
	for player in 0..P {
		if state.time_remaining[player] < max_rem - 5 {
			continue;
		}
		let mut bitfield = state.closed_valves;
		let mut new_loc = 0;
		while bitfield != 0 {
			if bitfield & 1 != 0 {
				let mut rem = state.time_remaining[player] - edges[[state.location[player], new_loc]];
				rem -= 1; // open valve time
				if rem > 0 {
					let mut s = state;
					s.closed_valves &= !(1 << new_loc);
					s.location[player] = new_loc;
					s.time_remaining[player] = rem;
					let released_by_valve = valves[new_loc].flow_rate * rem as usize;
					v.push(released_by_valve + visit(valves, edges, s));
				}
			}
			bitfield >>= 1;
			new_loc += 1;
		}
	}

	v.0
}

#[aoc(day16, part1)]
fn day16_part1(valves: &[Valve]) -> usize {
	solve::<1>(valves)
}
#[aoc(day16, part2)]
fn day16_part2(valves: &[Valve]) -> usize {
	solve::<2>(valves)
}

fn solve<const P: usize>(valves: &[Valve]) -> usize {
	let start_loc = 0;

	let mut edges = Grid::blank(&(valves.len(), valves.len()), i32::MAX);
	for (i, v) in valves.iter().enumerate() {
		for &c in v.children.iter() {
			edges[[i, c as usize]] = 1
		}
	}
	for i in 0..valves.len() {
		edges[[i, i]] = 0
	}
	for k in 0..valves.len() {
		for i in 0..valves.len() {
			for j in 0..valves.len() {
				edges[[i, j]] = edges[[i, j]].min(edges[[i, k]].saturating_add(edges[[k, j]]));
			}
		}
	}

	/*edges.print_mapped(|x| match *x {
		i32::MAX => ' ',
		v if v >= 0 && v <= 9 => (v as u8 + b'0') as char,
		_ => '+'
	});*/

	let closed_valves = valves.iter().enumerate()
		.map(|(i, v)| if v.flow_rate > 0 { 1 } else { 0 } << i)
		.reduce(|a, b| a | b)
		.unwrap();
	
	let remaining = if P == 1 { 30 } else { 26 };
	let state = State::<P>{
		closed_valves,
		location: [start_loc; P],
		time_remaining: [remaining; P],
	};

	// we do not handle starting on a node with a flow rate, as flow rate is handled
	// by moving to a target node
	assert_eq!(valves[start_loc].flow_rate, 0);

	visit(&valves, &edges, state)
}


#[cfg(test)]
const EXAMPLE_1: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

#[test]
fn test() {
	let ex1 = parse(EXAMPLE_1);
	assert_eq!(day16_part1(&ex1), 1651);
	assert_eq!(day16_part2(&ex1), 1707);
}