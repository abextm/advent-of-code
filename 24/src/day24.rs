use std::collections::HashMap;
use regex::{Match, Regex};

type Key = [u8; 3];

enum Op {
	Constant(bool),
	And(Key, Key),
	Or(Key, Key),
	Xor(Key, Key),
}

fn cap_to_key(v: Option<Match<'_>>) -> Key {
	v.unwrap().as_str().as_bytes().try_into().unwrap()
}

fn calc(v: &HashMap<Key, Op>, k: &Key) -> bool {
	match v.get(k).unwrap() {
		Op::Constant(v) => *v,
		Op::And(a, b) => calc(v, a) & calc(v, b),
		Op::Or(a, b) => calc(v, a) | calc(v, b),
		Op::Xor(a, b) => calc(v, a) ^ calc(v, b),
	}
}

#[aoc(part1 = 36035961805936)]
fn part1(input: &str) -> impl std::fmt::Debug {
	let mut map = HashMap::new();
	let re = Regex::new(r"([a-z0-9]{3}) (AND|XOR|OR) ([a-z0-9]{3}) -> ([a-z0-9]{3})|([a-z0-9]{3}): (0|1)").unwrap();
	for line in input.lines() {
		if line.len() == 0 {
			continue;
		}

		let cap = re.captures(line).unwrap();
		let (res, op) = if let Some(op) = cap.get(2) {
			let a = cap_to_key(cap.get(1));
			let b = cap_to_key(cap.get(3));
			(4, match op.as_str() {
				"AND" => Op::And(a, b),
				"OR" => Op::Or(a, b),
				"XOR" => Op::Xor(a, b),
				_ => panic!(),
			})
		} else {
			(5, Op::Constant(cap.get(6).unwrap().as_str() == "1"))
		};
		let res = cap_to_key(cap.get(res));
		map.insert(res, op);
	}

	let mut out = 0;
	map.keys()
		.filter(|v| v[0] == b'z')
		.for_each(|key| {
			let bit = (key[1] - b'0') * 10 + (key[2] - b'0');
			out |= (calc(&map, key) as u64) << bit;
		});

	out
}


const fn k2(k: u8, v: u8) -> Key2 {
	Key2::Known([
		k,
		(v / 10) + b'0',
		(v % 10) + b'0',
	])
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Key2 {
	Sym(u8),
	Known(Key),
}

#[derive(Debug)]
struct Op2 {
	op: u8,
	lhs: Key2,
	rhs: Key2,
	res: Key2,
}

struct GraphPair {
	expected: Vec<Op2>,
	test: HashMap<(Key, Key, u8), Op2>,
	num_syms: usize,
}

impl GraphPair {
	fn find(&self) -> Option<Vec<(Key, Key)>> {
		if let Some(r) = self.test_swaps(&mut vec![None; self.num_syms], 0, 0, &[]) {
			return Some(r);
		}
		None
	}

	fn test_swaps(&self, mapping: &mut [Option<Key>], start: usize, min_complete: usize, swaps: &[(Key, Key)]) -> Option<Vec<(Key, Key)>> {
		for (i, ex) in self.expected.iter().enumerate().skip(start) {
			let args = [ex.lhs, ex.rhs].map(|l| match l {
				Key2::Sym(v) => match mapping[v as usize] {
					Some(v) => v,
					None => panic!(),
				}
				Key2::Known(k) => k,
			});

			if let Some(test_op) = self.test.get(&(args[0], args[1], ex.op))
				.or_else(|| self.test.get(&(args[1], args[0], ex.op))) {
				if let Key2::Known(mut res) = test_op.res {
					for &(f, r) in swaps.iter() {
						if res == f {
							res = r;
							break;
						} else if res == r {
							res = f;
							break;
						}
					}

					match ex.res {
						Key2::Sym(sym) => {
							mapping[sym as usize] = Some(res);
							continue;
						}
						Key2::Known(k) => {
							if k == res {
								continue;
							}

							if swaps.len() == 4 || i <= min_complete {
								return None;
							}

							{
								let mut swaps = Vec::from(swaps);
								swaps.push((k, res));
								if let Some(v) = self.test_swaps(mapping, i, i, &swaps) {
									return Some(v);
								}
							}
						}
					}
				} else {
					panic!();
				}
			}

			if swaps.len() == 4 || i <= min_complete {
				return None;
			}

			let mut swaps = Vec::from(swaps);
			for swi in (0..i).rev() {
				if let Key2::Sym(sym) = self.expected[swi].res {
					let k1 = mapping[sym as usize].unwrap();
					for op2 in self.test.values() {
						if let Key2::Known(k2) = op2.res {
							if !mapping[..sym as usize].contains(&Some(k2)) && k1 != k2 {
								swaps.push((k1, k2));
								if let Some(v) = self.test_swaps(&mut mapping[..], swi, i, &swaps) {
									return Some(v);
								}
								swaps.pop();
							}
						}
					}
				}
			}

			return None
		}

		Some(swaps.to_vec())
	}
}

#[aoc(part2 = "\"jqf,mdd,skh,wpd,wts,z11,z19,z37\"")]
fn part2(input: &str) -> impl std::fmt::Debug {
	let re = Regex::new(r"([a-z0-9]{3}) (AND|XOR|OR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();
	let mut ops = HashMap::new();
	for line in input.trim().split("\n\n").skip(1).next().unwrap().lines() {
		let cap = re.captures(line).unwrap();
		let a = cap_to_key(cap.get(1));
		let b = cap_to_key(cap.get(3));
		let opbyte = cap.get(2).unwrap().as_str().as_bytes()[0];
		let op = Op2 {
			op: opbyte,
			lhs: Key2::Known(a),
			rhs: Key2::Known(b),
			res: Key2::Known(cap_to_key(cap.get(4))),
		};
		ops.insert((a, b, opbyte), op);
	}

	let n_bits = ops.values().filter_map(|op| {
		if let Key2::Known(key) = op.res {
			if key[0] == b'z' {
				return Some((key[1] - b'0') * 10 + (key[2] - b'0'));
			}
		}
		None
	}).max().unwrap();

	let mut expected = Vec::new();
	expected.push(Op2 {
		op: b'X',
		lhs: k2(b'x', 0),
		rhs: k2(b'y', 0),
		res: k2(b'z', 0),
	});
	expected.push(Op2 {
		op: b'A',
		lhs: k2(b'x', 0),
		rhs: k2(b'y', 0),
		res: Key2::Sym(0),
	});
	for bit in 1..n_bits {
		let soff = (bit - 1) * 4;
		let carry = if bit == n_bits - 1 {
			k2(b'z', bit + 1)
		} else {
			Key2::Sym(soff + 4)
		};
		expected.push(Op2 {
			op: b'X',
			lhs: k2(b'x', bit),
			rhs: k2(b'y', bit),
			res: Key2::Sym(soff + 1),
		});
		expected.push(Op2 {
			op: b'X',
			lhs: Key2::Sym(soff),
			rhs: Key2::Sym(soff + 1),
			res: k2(b'z', bit),
		});
		expected.push(Op2 {
			op: b'A',
			lhs: k2(b'x', bit),
			rhs: k2(b'y', bit),
			res: Key2::Sym(soff + 2),
		});
		expected.push(Op2 {
			op: b'A',
			lhs: Key2::Sym(soff),
			rhs: Key2::Sym(soff + 1),
			res: Key2::Sym(soff + 3),
		});
		expected.push(Op2 {
			op: b'O',
			lhs: Key2::Sym(soff + 2),
			rhs: Key2::Sym(soff + 3),
			res: carry,
		});
	}

	let gp = GraphPair {
		expected,
		test: ops,
		num_syms: n_bits as usize * 4 + 1,
	};


	let swaps = gp.find().unwrap();

	let mut sw = swaps.iter()
		.flat_map(|v| [&v.0, &v.1].into_iter())
		.map(|b| std::str::from_utf8(b.as_slice()).unwrap())
		.collect::<Vec<_>>();
	sw.sort();
	sw.join(",")
}