use crate::grid::Grid;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::mem::swap;

type RockID = u16;
type Dim = u8;
struct Shifter<const NY: bool, const TRANSPOSED: bool> {
	rocks: Vec<RockID>,
	rocks_stride: usize,
	rock_x: Vec<Dim>,
	rock_y_initial: Vec<Dim>,
	rock_y_value: Vec<Dim>,
}

fn range_step_generic<const NX: bool, const INV: bool>(min: isize, max: isize) -> num::iter::RangeStepInclusive<isize> {
	if NX ^ INV {
		num::range_step_inclusive(max - 1, min, -1)
	} else {
		num::range_step_inclusive(min, max - 1, 1)
	}
}

impl<const NY: bool, const TRANSPOSED: bool> Shifter<NY, TRANSPOSED> {
	const DY: isize = if NY { -1 } else { 1 };

	fn from(mut grid: Grid<&[u8]>) -> Self {
		if TRANSPOSED {
			grid = grid.transposed();
		}

		let rocks_stride = grid.width();
		let mut rocks = vec![0 as RockID; rocks_stride * grid.height()];
		let mut rock_y = Vec::new();
		let mut rock_x = Vec::new();

		for x in 0..grid.width() as isize {
			let mut used = false;
			for y in range_step_generic::<NY, true>(0, grid.height() as isize) {
				rocks[x as usize + y as usize * rocks_stride] = if grid[[x, y]] == b'#' {
					if used {
						rock_y.push((y + Self::DY) as Dim);
						rock_x.push(x as Dim);
						used = false;
					}
					u16::MAX
				} else {
					used = true;
					rock_y.len() as RockID
				};
			}
			if used {
				rock_y.push(if NY { grid.height() as Dim - 1 } else { 0 });
				rock_x.push(x as Dim);
			}
		}
		
		Shifter {
			rocks,
			rocks_stride,
			rock_x,
			rock_y_initial: rock_y.clone(),
			rock_y_value: rock_y
		}
	}

	fn shift(&mut self, mut point: (Dim, Dim)) -> (Dim, Dim) {
		if TRANSPOSED {
			point = (point.1, point.0);
		}

		let rock = self.rocks[point.0 as usize + point.1 as usize * self.rocks_stride];
		debug_assert_ne!(rock, u16::MAX);
		let ry = &mut self.rock_y_value[rock as usize];
		point.1 = *ry as Dim;
		*ry = (Self::DY + *ry as isize) as Dim;

		if TRANSPOSED {
			point = (point.1, point.0);
		}
		point
	}

	fn shift_all(&mut self, points: &mut [(Dim, Dim)]) {
		for pt in points.iter_mut() {
			*pt = self.shift(*pt);
		}
	}

	fn reset(&mut self) {
		self.rock_y_value.copy_from_slice(&self.rock_y_initial);
	}
}

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
	let grid = Grid::from_char_grid(input);

	let mut shifter = Shifter::<false, false>::from(grid);
	let mut points = grid.filter_enumerate(|&x| x == b'O')
		.map(|(x, y, _)| (x as Dim, y as Dim))
		.collect::<Vec<_>>();

	shifter.shift_all(&mut points);

	points.iter()
		.map(|&(_x, y)| grid.height() - y as usize)
		.sum()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
	let grid = Grid::from_char_grid(input);

	let mut shifters = (
		Shifter::<false, false>::from(grid),
		Shifter::<false, true>::from(grid),
		Shifter::<true, false>::from(grid),
		Shifter::<true, true>::from(grid),
	);
	let mut points = grid.filter_enumerate(|&x| x == b'O')
		.map(|(x, y, _)| (x as Dim, y as Dim))
		.collect::<Vec<_>>();

	let mut history = HashMap::<Vec<Dim>, usize>::new();

	let target_cycles = 1_000_000_000;
	let mut cycle = 0;

	while cycle < target_cycles {
		shifters.0.shift_all(&mut points);
		shifters.1.shift_all(&mut points);
		shifters.2.shift_all(&mut points);
		shifters.3.shift_all(&mut points);

		cycle += 1;

		let mut v = shifters.3.rock_y_initial.clone();
		swap(&mut v, &mut shifters.3.rock_y_value);

		match history.entry(v) {
			Entry::Occupied(v) => {
				let last_cycle = v.get();
				let delta = cycle - last_cycle;
				cycle += (target_cycles - cycle) / delta * delta;
				let first_target_cycle = last_cycle + (target_cycles - cycle);

				// shifter 3's eastward view, so backwards and transposed
				let rock_y = history.iter()
					.find(|(_k, &v)| v == first_target_cycle)
					.unwrap()
					.0;

				return shifters.3.rock_x.iter().zip(shifters.3.rock_y_initial.iter().zip(rock_y.iter()))
					.map(|(&x, (&y_init, &y))| (grid.height() - x as usize) * (y_init - y) as usize)
					.sum()
			},
			Entry::Vacant(v) => {
				v.insert(cycle);
			},
		}

		shifters.0.reset();
		shifters.1.reset();
		shifters.2.reset();
		// 3 was reset via map insert swap
	}

	panic!();
}

#[cfg(test)]
const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[test]
fn test_p1() {
	assert_eq!(136, part1(EXAMPLE));
}

#[test]
fn test_p2() {
	assert_eq!(64, part2(EXAMPLE));
}