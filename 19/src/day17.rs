use crate::{grid, vm};
use std::collections::{HashSet, HashMap};
use crate::day17::tile::STRUCTURE;
use crate::grid::Grid;

mod tile {
	pub const NEWLINE: u8 = b'\n';
	pub const STRUCTURE: u8 = b'#';
	pub const START: u8 = b'^';
	pub const SPACE: u8 = b'.';
}

const DIRS: [(isize, isize); 4] = [
	(0, 1),
	(1, 0),
	(0, -1),
	(-1, 0),
];

#[aoc(part1=5680)]
fn day17_part1(input: &str) -> usize {
	let state = vm::new_from_str(input).unwrap();
	let map = state.map(|x| x.unwrap() as u8).collect::<Vec<_>>();
	let map = Grid::from_char_grid(&map);
	map.iter()
		.filter(|&(x, y, &t)| t == tile::STRUCTURE && grid::ADJ4.iter().all(|(dx, dy)| map.get(x as isize + dx, y as isize + dy) == Some(&STRUCTURE)))
		.map(|(x, y, _)| x * y)
		.sum()
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

#[aoc(part2=895965)]
fn day17_part2(input: &str) -> i64 {
	let mut state = vm::new_from_str(input).unwrap();
	let map = state.clone()
		.map(|x| x.unwrap() as u8)
		.collect::<Vec<_>>();
	let map = Grid::from_char_grid(&map);

	let start = map
		.filter_enumerate(|&t| t == tile::START)
		.next()
		.expect("no start");
	let start = (start.0 as isize, start.1 as isize);

	// inputs seem to have the trait that at an intersection you always continue straight
	// if this were not true we would have to do actual pathfinding

	let path = {
		let mut path = Vec::new();
		let mut coord = start;

		let mut dir = DIRS
			.iter()
			.enumerate()
			.filter(|(_, d)| map.get(start.0 + d.0, start.1 + d.1).cloned() == Some(tile::STRUCTURE))
			.next()
			.expect("no structure by start")
			.0;
		
		let mut active = (dir, 0);

	'outer:
		loop {
			for newdir in &[
				0, // FOREWARD
				3, // LEFT
				1, // RIGHT
			] {
				let newdir = (dir + newdir) % 4;
				let newcoord = add2(coord, DIRS[newdir]);
				if map.get(newcoord.0, newcoord.1) == Some(&tile::STRUCTURE) {
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

	//print!("{}", map.iter().map(|x| std::char::from_u32(*x as u32).unwrap()).collect::<String>());

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
