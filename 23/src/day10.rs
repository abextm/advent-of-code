use crate::grid;

const DIR_N: u8 = 0b1000;
const DIR_E: u8 = 0b0100;
const DIR_S: u8 = 0b0010;
const DIR_W: u8 = 0b0001;
const DIR_START: u8 = DIR_N | DIR_E | DIR_S | DIR_W;

fn invert_dir(dir: u8) -> u8 {
	match dir {
		DIR_W => DIR_E,
		DIR_E => DIR_W,
		DIR_N => DIR_S,
		DIR_S => DIR_N,
		v => panic!("{}", v),
	}
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
	let grid = grid::Grid::from_str_with_mapper(input, |&s| match s {
		b'|' => DIR_N | DIR_S,
		b'-' => DIR_E | DIR_W,
		b'L' => DIR_N | DIR_E,
		b'J' => DIR_N | DIR_W,
		b'7' => DIR_S | DIR_W,
		b'F' => DIR_S | DIR_E,
		b'.' => 0,
		b'S' => DIR_START,
		v => panic!("{:?}", v as char),
	});

	let start = grid.find(&DIR_START).unwrap();

	let v = grid.adjacent4(start.0, start.1)
		.filter(|&(_x, _y, &v)| v != 0)
		.filter_map(|point| {
			let mut point = (point.0, point.1);
			let mut dir = match (start.0 as isize - point.0 as isize, start.1 as isize - point.1 as isize) {
				(1, 0) => DIR_W,
				(-1, 0) => DIR_E,
				(0, 1) => DIR_N,
				(0, -1) => DIR_S,
				_ => panic!(),
			};

			if grid[point] & invert_dir(dir) == 0 {
				return None;
			}

			let mut steps: usize = 0;

			loop {
				steps += 1;
				let from_dir = invert_dir(dir);
				dir = grid[point] & !from_dir;

				point = match dir {
					DIR_W => (point.0 - 1, point.1),
					DIR_E => (point.0 + 1, point.1),
					DIR_N => (point.0, point.1 - 1),
					DIR_S => (point.0, point.1 + 1),
					v => panic!("{}", v),
				};

				if point == start {
					break;
				}
			}

			Some((steps + 1) / 2)
		})
		.next().unwrap();

	v
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
	let grid = grid::Grid::from_str_with_mapper(input, |&s| match s {
		b'|' => DIR_N | DIR_S,
		b'-' => DIR_E | DIR_W,
		b'L' => DIR_N | DIR_E,
		b'J' => DIR_N | DIR_W,
		b'7' => DIR_S | DIR_W,
		b'F' => DIR_S | DIR_E,
		b'.' => 0,
		b'S' => DIR_START,
		v => panic!("{:?}", v as char),
	});

	let start = grid.find(&DIR_START).unwrap();

	let [(mut point, mut dir), (_, other_dir)] = grid.adjacent4(start.0, start.1)
		.filter(|&(_x, _y, &v)| v != 0)
		.filter_map(|point| {
			let point = (point.0, point.1);
			let dir = match (start.0 as isize - point.0 as isize, start.1 as isize - point.1 as isize) {
				(1, 0) => DIR_W,
				(-1, 0) => DIR_E,
				(0, 1) => DIR_N,
				(0, -1) => DIR_S,
				_ => panic!(),
			};

			if grid[point] & invert_dir(dir) == 0 {
				return None;
			}

			Some((point, dir))
		}).array_chunks::<2>()
			.next()
			.unwrap();

	let mut grid_out = grid.map(|_, _, _| 0 as u8);

	grid_out[start] = dir | other_dir;

	loop {
		grid_out[point] = grid[point];
		let from_dir = invert_dir(dir);
		dir = grid[point] & !from_dir;

		point = match dir {
			DIR_W => (point.0 - 1, point.1),
			DIR_E => (point.0 + 1, point.1),
			DIR_N => (point.0, point.1 - 1),
			DIR_S => (point.0, point.1 + 1),
			v => panic!("{}", v),
		};

		if point == start {
			break;
		}
	}

	let mut area = 0;
	for y in 0..grid.height() {
		let mut x = 0;
		let mut in0 = 0;
		while x < grid.width() {
			let v = grid_out[[x, y]];
			if v & (DIR_N | DIR_S) != 0 {
				in0 ^= v & (DIR_N | DIR_S)
			} else if in0 == (DIR_N | DIR_S) && v == 0 {
				grid_out[[x, y]] = 0b10000;
				area += 1;
			}
			x += 1;
		}
	}

	grid_out.print_mapped(|&v| if v == 0 {
		' '
	} else if v == DIR_N | DIR_S{
		'┃'
	} else if v == DIR_W | DIR_E{
		'━'
	} else if v == DIR_N | DIR_E{
		'┗'
	} else if v == DIR_N | DIR_W{
		'┛'
	} else if v == DIR_S | DIR_E{
		'┏'
	} else if v == DIR_S | DIR_W{
		'┓'
	} else if v == DIR_START{
		'S'
	} else if v == 0b10000{
		'░'
	} else {
		'*'
	});
	
	area
}

#[test]
fn test_p2() {
	assert_eq!(8, part2(".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."))
}