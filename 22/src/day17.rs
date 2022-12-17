use std::collections::hash_map::Entry;
use std::collections::{VecDeque, HashMap};


struct Rock {
	width: usize,
	bits: &'static [u8],
}

struct Cycle<'a, T> {
	s: &'a [T],
	p: usize,
}

impl<'a, T> Cycle<'a, T> {
	fn next(&mut self) -> &'a T {
		let pos = self.p;
		self.p = (self.p + 1) % self.s.len();
		&self.s[pos]
	}

	fn new(s: &'a [T]) -> Self {
		Cycle {
			s,
			p: 0,
		}
	}
}

struct TopGrid {
	rows: VecDeque<u8>,
	top: [isize; 7],
	bottom: isize,
	height: isize,
}

impl TopGrid {
	fn collides(&self, rock: &Rock, x: isize, y: isize) -> bool {
		if y > self.height {
			return false;
		}
		let ry = y - self.bottom;
		!rock.bits.iter().enumerate()
			.all(|(dy, line)| {
				let ry = ry + dy as isize;
				if ry < 0 {
					panic!()
				} else if ry >= self.rows.len() as isize {
					true
				} else {
					self.rows[ry as usize] & (line << x) == 0
				}
			})
	}

	fn insert(&mut self, rock: &Rock, x: isize, y: isize) {
		let top = (y + rock.bits.len() as isize) - self.bottom;
		while (self.rows.len() as isize) < top {
			self.rows.push_back(0);
		}
		let x = x as usize;

		rock.bits.iter().enumerate()
			.for_each(|(dy, line)| {
				let vy = y + dy as isize;
				self.rows[(vy - self.bottom) as usize] |= line << x;
				
				for b in 0..rock.width {
					if line & (1 << b) != 0 {
						self.top[x + b] = self.top[x + b].max(vy);
					}
				}
			});

		let min = *self.top.iter().min().unwrap() - 10;
		while self.bottom < min {
			self.rows.pop_front();
			self.bottom += 1;
		}
	}

	fn iterate(&mut self, rocks: &mut Cycle<Rock>, input: &mut Cycle<i8>) {
		let rock = rocks.next();
		let mut x = 2;
		let mut y = self.height + 3 as isize;
		let max_x = 7 - rock.width as isize;
		loop {
			let old_x = x;
			x += *input.next() as isize;

			if x < 0 || x > max_x || self.collides(&rock, x, y) {
				x = old_x;
			}

			y -= 1;

			let collides = y < 0 || self.collides(&rock, x, y);
			//println!("{:?} x={}", mov, x);
			if collides {
					y += 1;
					self.insert(&rock, x, y);
					self.height = self.height.max(y + rock.bits.len() as isize);

					/*println!("{}", self.bottom);
					for &v in self.rows.iter().rev() {
						for x in 0..7 {
							print!("{}", if v & 1 << x != 0 { '#' } else { '.' });
						}
						println!("");
					}*/
					return;
			}
		}
	}

	fn new() -> TopGrid {
		TopGrid {
			rows: VecDeque::new(),
			top: [0; 7],
			bottom: 0,
			height: 0,
		}
	}
}

fn convert_input(input: &str) -> Vec<i8> {
	input.chars()
		.map(|c| match c {
			'>' => 1,
			'<' => -1,
			_ => panic!(),
		}).collect()
}

#[aoc(day17, part1)]
fn day17_part1(input: &str) -> isize {
	let mut rocks = Cycle::new(ROCKS);
	let input = convert_input(input);
	let mut input = Cycle::new(&input);
	let mut state = TopGrid::new();
	for _ in 0..2022 {
		state.iterate(&mut rocks, &mut input);
	}
	state.height
}

#[aoc(day17, part2)]
fn day17_part2(input: &str) -> isize {
	let iterations = 1000000000000;
	let mut rocks = Cycle::new(ROCKS);
	let input = convert_input(input);
	let mut input = Cycle::new(&input);

	let mut state = TopGrid::new();

	let mut seen: HashMap<(usize, usize, [isize; 7]), (usize, isize)> = HashMap::new();
	let mut it = 0;
	while it < iterations {
		state.iterate(&mut rocks, &mut input);
		it += 1;

		let h = state.height;
		match seen.entry((rocks.p, input.p, state.top.map(|t| state.height - t))) {
			Entry::Occupied(e) => {
				let &(low_it, low_h) = e.get();
				let step = it - low_it;
				let steps = (iterations - it) / step;
				it += step * steps;
				let extra_height = (h - low_h) * steps as isize;

				while it < iterations {
					state.iterate(&mut rocks, &mut input);
					it += 1;
				}
				
				return state.height + extra_height;
			},
			Entry::Vacant(e) => { e.insert((it, h)); },
		};
	}

	println!("no cycle");
	state.height
}


const ROCKS: &[Rock] = &[
	Rock{
		width: 4,
		bits: &[0b1111],
	},
	Rock {
		width: 3,
		bits: &[
			0b010,
			0b111,
			0b010,
		],
	},
	Rock {
		width: 3,
		bits: &[
			0b111,
			0b100,
			0b100,
		],
	},
	Rock {
		width: 1,
		bits: &[1, 1, 1, 1],
	},
	Rock {
		width: 2,
		bits: &[
			0b11,
			0b11,
		],
	},
];

#[test]
fn test() {
	const EXAMPLE_1: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

	assert_eq!(day17_part1(EXAMPLE_1), 3068);
	assert_eq!(day17_part2(EXAMPLE_1), 1514285714288);
}
