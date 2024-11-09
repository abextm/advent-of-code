use crate::vm;
use std::collections::{HashSet, HashMap};

mod tile {
	pub const NEWLINE: i64 = '\n' as i64;
	pub const STRUCTURE: i64 = '#' as i64;
	pub const START: i64 = '^' as i64;
	pub const SPACE: i64 = '.' as i64;
}

mod command {
	pub const RIGHT: u8 = 'R' as u8;
	pub const LEFT: u8 = 'L' as u8;
	pub const FOREWARD: u8 = 'F' as u8;
}

const DIRS: [(isize, isize); 4] = [
	(0, 1),
	(1, 0),
	(0, -1),
	(-1, 0),
];

#[aoc(day17, part1)]
fn day17_part1(input: &str) -> usize {
	let mut state = vm::new_from_str(input).unwrap();
	let map = state.map(|x| x.unwrap()).collect::<Vec<_>>();
	let width = map
		.iter()
		.position(|x| *x == tile::NEWLINE)
		.expect("no newlines");
	let stride = width + 1;

	let mut alignment = 0;
	for y in 1..(map.len() / stride) - 1 {
		for x in 1..(width - 1) {
			let coord = x + (y * stride);
			if map[coord] == tile::STRUCTURE
				&& map[coord - 1] == tile::STRUCTURE
				&& map[coord + 1] == tile::STRUCTURE
				&& map[coord - stride] == tile::STRUCTURE
				&& map[coord + stride] == tile::STRUCTURE
			{
				alignment += x * y;
			}
		}
	}

	alignment
}

fn get_grid<T>(grid: &[T], stride: usize, coord: (isize, isize)) -> Option<&T> {
	let ystr = coord.1 as usize * stride;
	if coord.0 < 0 || coord.1 < 0 || coord.0 > stride as isize - 1 || ystr + stride > grid.len() {
		None
	} else {
		Some(&grid[ystr + coord.0 as usize])
	}
}

fn add2<T: std::ops::Add>(a: (T, T), b: (T, T)) -> (<T as std::ops::Add>::Output, <T as std::ops::Add>::Output) {
	(a.0 + b.0, a.1 + b.1)
}

#[aoc(day17, part2)]
fn day17_part2(input: &str) -> i64 {
	let mut state = vm::new_from_str(input).unwrap();
	let map = state.clone()
		.map(|x| x.unwrap())
		.collect::<Vec<_>>();
	let width = map
		.iter()
		.position(|x| *x == tile::NEWLINE)
		.expect("no newlines");
	let stride = width + 1;

	let start = map.iter()
		.position(|x| *x == tile::START)
		.expect("no start");
	let start = ((start % stride) as isize, (start / stride) as isize);

	// inputs seem to have the trait that at an intersection you always continue straight
	// if this were not true we would have to do actual pathfinding

	let path = {
		let mut path = Vec::new();
		let mut coord = start;

		let mut dir = DIRS
			.iter()
			.enumerate()
			.filter(|(_, d)| get_grid(&map, stride, add2(start, **d)).cloned() == Some(tile::STRUCTURE))
			.next()
			.expect("no structure by start")
			.0;
		
		let mut active = (dir, 0);

	'outer:
		loop {
			for &(newdir, cmd) in &[
				(0, command::FOREWARD),
				(3, command::LEFT),
				(1, command::RIGHT)
			] {
				let newdir = (dir + newdir) % 4;
				let mut newcoord = add2(coord, DIRS[newdir]);
				if get_grid(&map, stride, newcoord) == Some(&tile::STRUCTURE) {
					active.1 += 1;
					if dir != newdir {
						path.push(active);
					}
					dir = newdir;
					active = (dir, 1);
					coord = newcoord;
					continue 'outer;
				}
			}
			path.push(active);
			break
		}
		path.into_iter()
			.map(|(dir, count)| (dir, count, 1 + (count as f64).log10().ceil() as usize))
			.collect::<Vec<_>>()
	};

	let mut uses = HashMap::<(usize, usize, usize), HashSet<usize>>::new();
	for (i, v) in path.iter().enumerate() {
		uses.entry(*v)
			.or_default()
			.insert(i);
	}

	fn solution(
		uses: HashMap::<(usize, usize, usize), HashSet<usize>>,
		path: Vec<(usize, usize, usize)>,
		i: usize,
	) -> Option<String> {
		
		None
	}

	print!("{:?}", solution(uses, path, 0));

//	print!("{}", map.iter().map(|x| std::char::from_u32(*x as u32).unwrap()).collect::<String>());
	

	if true { return 0; }

	state.memory[0] = 2;
	for c in state
		.with_input(
			"\
A,B,A,C,A,B,C,A,B,C
R,12,R,4,R,10,R,12
R,6,L,8,R,10
L,8,R,4,R,4,R,6
n
"
			.chars()
			.map(|x| x as i64),
		) {
			match c {
				Ok(v) => {
					if v < 128 {
						print!("{}", std::char::from_u32(v as u32).unwrap());
					} else {
						return v;
					}
				}
				Err(v) => {
					print!("{:?}", v);
					return 0;
				}
			}
		}

	0
}
