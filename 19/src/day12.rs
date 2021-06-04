use crate::taken::TakeN;
use num;
use regex::Regex;
use std::fmt;

#[aoc(day12, part1)]
fn day12_part1(input: &str) -> isize {
	let mut system = System::new(input);
	for _ in 0..1000 {
		system.tick();
	}
	system.energy()
}

#[aoc(day12, part2)]
fn day12_part2(input: &str) -> isize {
	let mut system = System::new(input);
	let start = system.get_per_axis();
	let mut results = start.iter().map(|_| None).collect::<Vec<_>>();
	for i in 1.. {
		system.tick();

		let now = system.get_per_axis();
		let done = results
			.iter_mut()
			.zip(start.iter().zip(now.iter()))
			.filter_map(|(r, (s, n))| {
				if let None = r {
					if s == n {
						*r = Some(i);
					}
				}
				*r
			})
			.collect::<Vec<_>>();

		if done.len() == start.len() {
			return done.iter().skip(1).fold(done[0], |a, b| num::integer::lcm(a, *b))
		}
	}
	unreachable!();
}

#[derive(Clone, PartialEq)]
struct System {
	loc: Vec<[isize; 3]>,
	vel: Vec<[isize; 3]>,
}

impl System {
	fn new(input: &str) -> System {
		lazy_static! {
			static ref R: Regex =
				Regex::new("<x= *(-?[0-9]+) *, *y= *(-?[0-9]+) *, *z= *(-?[0-9]+) *>").unwrap();
		}
		let loc: Vec<_> = R
			.captures_iter(input)
			.map(|c| {
				c.iter()
					.skip(1)
					.map(|p| p.unwrap().as_str().parse().unwrap())
					.take_n()
					.unwrap()
			})
			.collect();
		let mut vel = Vec::new();
		vel.resize(loc.len(), [0; 3]);
		System { loc, vel }
	}

	fn tick(&mut self) {
		for (loc, vel) in self.loc.iter().zip(self.vel.iter_mut()) {
			for body in self.loc.iter() {
				loc
					.iter()
					.zip(body.iter())
					.map(|(a, b)| match b.cmp(a) {
						std::cmp::Ordering::Less => -1,
						std::cmp::Ordering::Equal => 0,
						std::cmp::Ordering::Greater => 1,
					})
					.zip(vel.iter_mut())
					.for_each(|(d, v)| *v += d)
			}
		}

		for (loc, vel) in self.loc.iter_mut().zip(self.vel.iter()) {
			loc.iter_mut().zip(vel.iter()).for_each(|(l, v)| *l += v);
		}
	}

	fn get_per_axis(&self) -> [Vec<(isize, isize)>; 3] {
		let cap = self.loc.len();
		let mut vec = [
			Vec::with_capacity(cap),
			Vec::with_capacity(cap),
			Vec::with_capacity(cap),
		];
		for (loc, vel) in self.loc.iter().zip(self.vel.iter()) {
			for (v, t) in vec
				.iter_mut()
				.zip(loc.iter().cloned().zip(vel.iter().cloned()))
			{
				v.push(t);
			}
		}
		vec
	}

	fn energy(&self) -> isize {
		self
			.loc
			.iter()
			.zip(self.vel.iter())
			.map(|(loc, vel)| {
				loc.iter().map(|n| n.abs()).sum::<isize>() * vel.iter().map(|n| n.abs()).sum::<isize>()
			})
			.sum::<isize>()
	}
}

impl fmt::Debug for System {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for (loc, vel) in self.loc.iter().zip(self.vel.iter()) {
			writeln!(
				f,
				"pos=<x= {}, y={}, z={}> vel=<x= {}, y= {}, z= {}>",
				loc[0], loc[1], loc[2], vel[0], vel[1], vel[2]
			)?;
		}
		Ok(())
	}
}

/*
#[aoc(day12, part2)]
fn day12_part2(input: &str) -> i32 {
	day12(input, true)
}*/

#[test]
fn example() {
	let mut s = System::new(
		"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
	);
	assert_eq!(s.energy(), 0);
	s.tick();
	assert_eq!(s.energy(), 229);
	for _ in 1..10 {
		s.tick();
	}
	println!("\n\n{:?}", s);
	assert_eq!(s.energy(), 179);
}
