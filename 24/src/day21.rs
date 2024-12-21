use std::collections::HashMap;
use std::io::Read;
use crate::grid::Ve;

#[derive(Clone)]
struct Arm {
	locs: HashMap<u8, Ve<2>>,
	next: Option<Box<Arm>>,
	memo: HashMap<Vec<u8>, usize>,
}

impl Arm {
	fn new(a: impl Into<Ve<2>>) -> Self {
		let ve = a.into();
		Arm {
			locs: HashMap::new(),
			next: None,
			memo: HashMap::new(),
		}.btn(b'A', ve)
	}

	fn btn(mut self, code: u8, pos: impl Into<Ve<2>>) -> Self {
		self.locs.insert(code, pos.into());
		self
	}

	fn minset(&mut self, btns: Vec<u8>) -> usize {
		if let Some(v) = self.memo.get(&btns) {
			*v
		} else {
			let mut sum = 0;

			let mut pt = *self.locs.get(&b'A').unwrap();

			for &v in btns.iter() {
				sum += self.min(&mut pt, v)
			}

			self.memo.insert(btns.clone(), sum);
			sum
		}
	}

	fn min(&mut self, pos: &mut Ve<2>, to: u8) -> usize {
		let &pt = self.locs.get(&to).unwrap();

		let from = *pos;
		*pos = pt;

		let delta = from - pt;
		let lr = if delta[0] < 0 { b'>' } else { b'<' };
		let ud = if delta[1] < 0 { b'v' } else { b'^' };

		let ns = [
			(delta[0].abs(), lr),
			(delta[1].abs(), ud),
		];

		let m = [0, 1].into_iter().filter_map(|mode| {
			let mut ns = ns;
			if mode == 1 {
				ns.swap(0, 1);
				// y then x
				if !self.locs.values().any(|ipt| *ipt == Ve([from[0], pt[1]])) {
					return None;
				}
			} else {
				// x then y
				if !self.locs.values().any(|ipt| *ipt == Ve([pt[0], from[1]])) {
					return None;
				}
			}
			let aeu = (0..(ns[0].0)).map(move |_| ns[0].1)
				.chain((0..(ns[1].0)).map(move |_| ns[1].1))
				.chain([b'A'].into_iter())
				.collect::<Vec<_>>();
			Some(if let Some(n) = &mut self.next {
				n.minset(aeu)
			} else {
				aeu.len()
			})
		}).min_by_key(|a| *a).unwrap();

		m
	}
}


#[aoc(part1 = 184716, part2 = 229403562787554)]
fn solve(input: &str, part1: bool) -> impl std::fmt::Debug {
	let mut aa = Arm::new([3, 4])
		.btn(b'7', [1, 1])
		.btn(b'8', [2, 1])
		.btn(b'9', [3, 1])
		.btn(b'4', [1, 2])
		.btn(b'5', [2, 2])
		.btn(b'6', [3, 2])
		.btn(b'1', [1, 3])
		.btn(b'2', [2, 3])
		.btn(b'3', [3, 3])
		.btn(b'0', [2, 4]);

	let directional = Arm::new([3, 1])
		.btn(b'<', [1, 2])
		.btn(b'v', [2, 2])
		.btn(b'>', [3, 2])
		.btn(b'^', [2, 1]);

	let n = if part1 { 2 } else { 25 };

	for i in 0..n {
		let mut new = directional.clone();
		let mut old = None;
		std::mem::swap(&mut old, &mut aa.next);
		new.next = old;
		aa.next = Some(Box::new(new));
	}

	input.trim().lines().map(|l| {
		let numeric = l[..3].parse::<usize>().unwrap();

		let min = aa.minset(l.bytes().collect());
		//println!("{} {}", numeric, min);
		//println!("{}", aa.next.as_mut().unwrap().next.as_mut().unwrap().record);
		//println!("{}", aa.next.as_mut().unwrap().record);
		//println!("{}", aa.record);
		min * numeric
	}).sum::<usize>()
}

#[aoc(part1 = 126384)]
const EX: &str = "029A
980A
179A
456A
379A
";
