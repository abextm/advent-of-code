use std::collections::HashSet;
use crate::grid;
use crate::grid::Grid;

#[aoc(part1=18371095)]
fn part1(input: &str) -> impl std::fmt::Debug {
	let mut map: u32 = Grid::from_char_grid(input).iter().map(|(x, y, &v)| ((v == b'#') as u32) << (y * 5 + x)).sum();

	let mut seen = HashSet::new();

	let masks = (0..5)
		.flat_map(|y| (0..5)
			.map(move |x| grid::adjacent4_points(&(5, 5), x, y)
				.map(move |(x, y)| 1 << (x + y * 5))
				.sum()))
		.collect::<Vec<u32>>();

	loop {
		if !seen.insert(map) {
			return map
		}

		let oldmap = map;
		for i in 0..25 {
			let adj_mask = masks[i];
			let n = (oldmap & adj_mask).count_ones();
			let cur = (oldmap & (1 << i)) != 0;

			if n != 1 && cur {
				map &= !(1 << i);
			} else if n >= 1 && n <= 2 && !cur {
				map |= 1 << i;
			}
		}

		Grid::from_generator(&(5, 5), |x, y| if map & 1 <<(x + (y * 5)) != 0 { b'#' } else { b'.' }).print_c();
		println!();
	}
}

#[aoc(part2)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let map: u32 = Grid::from_char_grid(input).iter().map(|(x, y, &v)| ((v == b'#') as u32) << (y * 5 + x)).sum();

	// current grid 0=>25, inner grid 25->50, outer center 50->61

	let masks = (0..5usize)
		.flat_map(|y| (0..5usize)
			.map(move |x| {
				let mut mask = 0u64;
				if x != 2 || y != 2 {
					for &(dx, dy) in grid::ADJ4.iter() {
						let ix = x.wrapping_add(dx as usize);
						let iy = y.wrapping_add(dy as usize);
						if ix == 2 && iy == 2 {
							let ranges = match (dx, dy) {
								(1, 0) => (0..1, 0..5),
								(-1, 0) => (4..5, 0..5),
								(0, 1) => (0..5, 0..1),
								(0, -1) => (0..5, 4..5),
								_ => panic!(),
							};
							for x in ranges.0 {
								for y in ranges.1.clone() {
									mask |= 1 << (25 + x + y * 5);
								}
							}
						} else if ix < 5 && iy < 5 {
							mask |= 1 << (ix + iy * 5);
						} else {
							mask |= 1 << (1 + dx + (11 + dy) * 5);
						}
					}
				}

				mask
			}))
		.collect::<Vec<u64>>();

	let mut oldgrid = Vec::new();
	let mut grid = vec![map];
	for _minute in 0..200 {
		std::mem::swap(&mut grid, &mut oldgrid);
		grid.clear();

		for level in -1..=(oldgrid.len() as isize) {
			let outer = oldgrid.get((level - 1) as usize).cloned().unwrap_or(0);
			let med = oldgrid.get(level as usize).cloned().unwrap_or(0);
			let inner = oldgrid.get((level + 1) as usize).cloned().unwrap_or(0);

			let oldmap = med as u64 | ((inner as u64) << 25) | ((outer as u64 >> 6) << 50);

			let mut map = med;
			for i in 0..25 {
				let adj_mask = masks[i];
				let n = (oldmap & adj_mask).count_ones();
				let cur = (oldmap & (1 << i)) != 0;

				if n != 1 && cur {
					map &= !(1 << i);
				} else if n >= 1 && n <= 2 && !cur {
					map |= 1 << i;
				}
			}

			//Grid::from_generator(&(5, 5), |x, y| if map & 1 << (x + (y * 5)).min(63) != 0 { b'#' } else { b'.' }).print_c();
			//println!("{}", level);

			if map != 0 || (level >= 0 && level < oldgrid.len() as isize) {
				grid.push(map);
			}
		}

		//println!();
	}

	grid.iter().map(|m| m.count_ones()).sum::<u32>()
}

#[aoc(part2=1922)]
const EXAMPLE: &str = "....#
#..#.
#..##
..#..
#....
";