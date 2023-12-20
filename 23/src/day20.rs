use std::{collections::{HashMap, VecDeque}, cell::Cell};

#[derive(Clone, Debug)]
enum Module {
	FlipFlop{
		state: bool,
		input_mask: Cell<u64>,
		dst: Vec<u8>,
	},
	And{
		state_bits: u64,
		input_mask: Cell<u64>,
		dst: Vec<u8>,
	},
	Broadcast{
		dst: Vec<u8>,
	},
	None,
}

impl Module {
	fn dest(&self) -> Option<&[u8]> {
		match self {
			Module::FlipFlop{ dst, ..} => Some(dst),
			Module::And{ dst, ..} => Some(dst),
			Module::Broadcast{ dst, ..} => Some(dst),
			Module::None => None,
		}
	}
	fn in_mask(&self) -> u64 {
		match self {
			Module::FlipFlop{ input_mask, ..} => input_mask.get(),
			Module::And{ input_mask, ..} => input_mask.get(),
			_ => 0,
		}
	}
}

fn parse_input(input: &str) -> Vec<Module> {
	let mut names = HashMap::<&str, u8>::new();
	names.insert("roadcaster", 0);
	names.insert("rx", 1);
	let mut nodes = vec![Module::None; 64];
	for line in input.lines() {
		let (name, line) = line.split_once(" -> ").unwrap();
		let dst = line.split(", ").map(|bit| {
			let count = names.len() as u8;
			*names.entry(bit).or_insert(count)
		}).collect::<Vec<_>>();
		let module = match name.as_bytes()[0] {
			b'%' => Module::FlipFlop{ state: false, input_mask: Cell::new(0), dst },
			b'&' => Module::And { state_bits: 0, input_mask: Cell::new(0), dst },
			b'b' => Module::Broadcast { dst },
			_ => panic!(),
		};

		let count = names.len() as u8;
		let key = *names.entry(&name[1..]).or_insert(count);
		nodes[key as usize] = module;
	}

	for (id, node) in nodes.iter().enumerate() {
		if let Some(dst) = node.dest() {
			for &nid in dst {
				if let Some(input_mask) = match &nodes[nid as usize] {
					Module::And{ input_mask, .. } => Some(input_mask),
					Module::FlipFlop { input_mask, .. } => Some(input_mask),
					_ => None,
				} {
					input_mask.set(input_mask.get() | (1 << id));
				}
			}
		}
	}

	nodes
}

fn eval(nodes: &mut [Module], pulse_q: &mut VecDeque<(u8, bool, u8)>, target: u8, is_hi: bool, src: u8) {
	let module = &mut nodes[target as usize];
	match module {
		Module::FlipFlop{state, dst, ..} if !is_hi => {
			*state ^= true;
			let state = *state;
			pulse_q.extend(dst.iter().map(|&t| (t, state, target)));
		},
		Module::FlipFlop{..} => {
		},
		Module::And { state_bits, input_mask, dst} => {
			*state_bits = (*state_bits & !(1 << src)) | ((is_hi as u64) << src);
			let matches = *state_bits == input_mask.get();
			assert_eq!(*state_bits & !input_mask.get(), 0, "state {:b} mask {:b} target {} src {}", *state_bits, input_mask.get(), target, src);
			pulse_q.extend(dst.iter().map(|&t| (t, !matches, target)));
		},
		Module::Broadcast { dst } => {
			pulse_q.extend(dst.iter().map(|&t| (t, is_hi, target)));
		},
		Module::None => {},
	}
}

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
	let mut nodes = parse_input(input);

	let mut pulse_count = [0usize; 2];
	let mut pulse_q = VecDeque::<(u8, bool, u8)>::new();

	for _ in 0..1000 {
		pulse_q.push_back((0, false, 0)); // broadcaster
		while let Some((target, is_hi, src)) = pulse_q.pop_front() {
			pulse_count[is_hi as usize] += 1;
			eval(&mut nodes, &mut pulse_q, target, is_hi, src);
		}
	}

	pulse_count[0] * pulse_count[1]
}

#[test]
fn test_p1_b() {
	assert_eq!(32000000, part1("broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"));
}

#[test]
fn test_p1_a() {
	assert_eq!(11687500, part1("broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"));
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
	let mut nodes = parse_input(input);
	let mut first_hi = Vec::new();

	let mut wanted_nodes = nodes.iter().find(|n| n.dest().map(|l| l.contains(&1)) == Some(true)).unwrap().in_mask();

	let mut pulse_q = VecDeque::<(u8, bool, u8)>::new();

	for press in 1.. {
		pulse_q.push_back((0, false, 0)); // broadcaster
		while let Some((target, is_hi, src)) = pulse_q.pop_front() {
			if is_hi && wanted_nodes & (1 << src) != 0 {
				wanted_nodes &= !(1 << src);
				first_hi.push(press);
				if wanted_nodes == 0 {
					return first_hi.into_iter().reduce(num::integer::lcm).unwrap();
				}
			}
			eval(&mut nodes, &mut pulse_q, target, is_hi, src);
		}
	}

	panic!();
}