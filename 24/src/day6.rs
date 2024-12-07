use std::collections::HashSet;
use crate::grid::{Grid, Ve};

#[aoc(part1=4454)]
fn part1(input: &str) -> impl std::fmt::Debug {
	let mut grid = Grid::from_char_grid(input).cloned();

	let start = grid.find(b'^').next().unwrap();
	let mut dir: Ve<2> = [0, -1].into();

	let mut count = 1;
	grid[start] = b'x';
	let mut pt = start;

	loop {
		let npt = pt + dir;
		match grid.get(npt) {
			Some(b'#') => {
				dir = [-dir[1], dir[0]].into();
				continue;
			},
			Some(b'.') => {
				grid[npt] = b'x';
				count += 1;
				pt = npt;
			},
			Some(b'x') => {
				if pt == start {
					break;
				}
				pt = npt;
			}
			None => break,
			_ => panic!(),
		}
	}

	//println!("{}", std::str::from_utf8(&grid.array).unwrap());

	count
}

const DIRS: [Ve<2>; 4] = [
	Ve([0, -1]),
	Ve([1, 0]),
	Ve([0, 1]),
	Ve([-1, 0]),
];

#[aoc(part2)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let grid = Grid::from_char_grid(input);

	let start = grid.find(b'^').next().unwrap();
	let mut pt = start;

	let mut new_obstructions = HashSet::<Ve<2>>::new();
	let mut dir = 0;
	loop {
		let npt = pt + DIRS[dir];
		match grid.get(npt) {
			Some(b'#') => {
				dir = (dir + 1) & 3;
				continue;
			}
			Some(_) => {
				let mut g2 = grid.cloned();
				g2[npt] = b'#';
				if does_loop(g2, start) {
					new_obstructions.insert(npt);
				}
				pt = npt;
			}
			None => break,
		}
	}
	//println!("{}", std::str::from_utf8(&grid.array).unwrap());

	//new_obstructions.remove(&start);

	new_obstructions.len()
}

fn does_loop(mut grid: Grid<2, Vec<u8>>, mut pt: Ve<2>) -> bool {
	let mut dir = 0;
	loop {
		let npt = pt + DIRS[dir];
		match grid.get(npt) {
			Some(b'#') => {
				dir = (dir + 1) & 3;
				continue;
			}
			Some(b'.') | Some(b'^') => {
				grid[npt] = 1 << dir;
			}
			Some(&n) if n < 16 => {
				if (n & (1 << dir)) != 0 {
					return true;
				}
				grid[npt] |= 1 << dir;
			}
			None => return false,
			v => panic!("{:?}", v),
		}
		pt = npt;
	}
}

#[aoc(part2=6)]
const EX: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";