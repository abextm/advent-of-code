use std::collections::{HashSet, VecDeque};
use crate::grid::{Grid, Ve};

#[aoc(part1 = 1479679)]
fn part1(input: &str) -> impl std::fmt::Debug {
	let [grid, moves] = input.trim().split("\n\n").next_chunk().unwrap();

	let mut grid = Grid::from_char_grid(grid).cloned();

	let mut pos = grid.find(b'@').next().unwrap();
	grid[pos] = b'.';

	'moves_it: for mov in moves.bytes() {
		let dir = match mov {
			b'\n' => continue,
			b'^' => Ve([0, -1]),
			b'v' => Ve([0, 1]),
			b'>' => Ve([1, 0]),
			b'<' => Ve([-1, 0]),
			_ => panic!(),
		};

		let new_pos = pos + dir;
		let mut end = new_pos;
		loop {
			match grid.get(end) {
				None | Some(b'#') => continue 'moves_it,
				Some(b'O') => {}
				Some(b'.') => break,
				_ => panic!(),
			}
			end = end + dir;
		}

		if end != new_pos {
			grid[end] = b'O';
			grid[new_pos] = b'.';
		}

		pos = new_pos;
	}

	grid.print();

	grid.find(b'O').map(|pt| pt[1] * 100 + pt[0]).sum::<isize>()
}

#[aoc(part2 = 1509780)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let [grid, moves] = input.trim().split("\n\n").next_chunk().unwrap();
	let grid = Grid::from_char_grid(grid);
	let mut pos = grid.find(b'@').next().unwrap() * Ve([2, 1]);
	let mut grid = Grid::new([grid.shape()[0] * 2, grid.shape()[1]], ()).map(|pt, ()| match grid[[pt[0] >> 1, pt[1]]] {
		b'@' => b'.',
		b'O' => if pt[0] & 1 == 0 { b'[' } else { b']' },
		v => v,
	});

	let mut push_edge = VecDeque::new();
	let mut to_push = Vec::new();
	let mut pushed = HashSet::new();

	'moves_it: for mov in moves.bytes() {
		let dir = match mov {
			b'\n' => continue,
			b'^' => Ve([0, -1]),
			b'v' => Ve([0, 1]),
			b'>' => Ve([1, 0]),
			b'<' => Ve([-1, 0]),
			_ => panic!(),
		};

		let new_pos = pos + dir;

		push_edge.clear();
		to_push.clear();
		pushed.clear();

		push_edge.push_back(new_pos);

		while let Some(pt) = push_edge.pop_front() {
			let other_pt = match grid[pt] {
				b'#' => continue 'moves_it,
				b'[' if dir[0] == 0 => Some(pt + Ve([1, 0])),
				b']' if dir[0] == 0 => Some(pt + Ve([-1, 0])),
				b'[' | b']' if dir[1] == 0 => None,
				b'.' => continue,
				_ => panic!(),
			};

			for v in [Some(pt), other_pt] {
				if let Some(mut v) = v {
					if pushed.insert(v) {
						to_push.push(v);
						v = v + dir;
						push_edge.push_back(v);
					}
				}
			}
		}

		for &v in to_push.iter().rev() {
			grid[v + dir] = grid[v];
			grid[v] = b'.';
		}

		pos = new_pos;
	}

	grid.print();

	grid.find(b'[').map(|pt| pt[1] * 100 + pt[0]).sum::<isize>()
}

#[aoc(part2 = 618)]
const EX_SMALL: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

#[aoc(part1 = 10092, part2 = 9021)]
const EX_LARGE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";