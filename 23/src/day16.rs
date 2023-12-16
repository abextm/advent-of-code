use crate::grid::Grid;

fn add(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
	(a.0 + b.0, a.1 + b.1)
}

fn beam(grid: &Grid<&[u8]>, energized: &mut Grid<Vec<u8>>, mut point: (isize, isize), mut dir: (isize, isize)) {
	while let Some(pt) = grid.get(point.0, point.1) {
		let dir_mask = match dir {
			(0, 1)  => 0b0001u8,
			(0, -1) => 0b0010,
			(1, 0)  => 0b0100,
			(-1, 0) => 0b1000,
			_ => panic!(),
		};

		if energized[point] & dir_mask != 0 {
			return;
		}
		energized[point] |= dir_mask;

		match *pt {
			b'.' => {
				point = add(point, dir);
			},
			b'|' => {
				if dir.0 != 0 { // left / right
					beam(grid, energized, add(point, (0, -1)), (0, -1));
					beam(grid, energized, add(point, (0, 1)), (0, 1));
					return;
				} else {
					point = add(point, dir);
				}
			},
			b'-' => {
				if dir.1 != 0 { // up / down
					beam(grid, energized, add(point, (-1, 0)), (-1, 0));
					beam(grid, energized, add(point, (1, 0)), (1, 0));
					return;
				} else {
					point = add(point, dir);
				}
			},
			b'/' => {
				dir = (-dir.1, -dir.0);
				point = add(point, dir);
			},
			b'\\' => {
				dir = (dir.1, dir.0);
				point = add(point, dir);
			},
			c => panic!("{:?}", c as char),
		}
	}
}

fn energized(grid: &Grid<&[u8]>, point: (isize, isize), dir: (isize, isize)) -> usize {
	let mut energized = Grid::blank(grid, 0u8);
	beam(&grid, &mut energized, point, dir);
	energized.iter()
		.filter(|&(_x, _y, &v)| v != 0)
		.count()
}

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
	let grid = Grid::from_char_grid(input);
	energized(&grid, (0, 0), (1, 0))
}

#[aoc(day16, part2)]
fn part2(input: &str) -> usize {
	let grid = Grid::from_char_grid(input);
	(0..grid.height()).map(|y| energized(&grid, (0, y as isize), (1, 0)).max(energized(&grid, (grid.width() as isize - 1, y as isize), (-1, 0)))).max().unwrap()
		.max((0..grid.width()).map(|x| energized(&grid, (x as isize, 0), (0, 1)).max(energized(&grid, (x as isize, grid.height() as isize - 1), (0, -1)))).max().unwrap())
}


#[test]
fn test_p1() {
	assert_eq!(46, part1(".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."));
}