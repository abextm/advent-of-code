use crate::grid::Grid;

fn advance(from: &Grid<u8>, next: &mut Grid<u8>, dx: usize, dy: usize, target: u8) -> usize {
	let mut count = 0;
	for (x, y, v) in from.iter() {
		if target == *v {
			let tx = (x + dx) % next.width();
			let ty = (y + dy) % next.height();;
			if from[[tx, ty]] == b'.' && next[[tx, ty]] == b'.' {
				next[[tx, ty]] = *v;
				count += 1;
				continue;
			}
		}
		if *v != b'.' {
			next[[x, y]] = *v;
		}
	}
	count
}

#[aoc(day25, part1)]
fn part1(input: &str) -> u64 {
	let mut grid = Grid::from_str_with_mapper(input, |x| *x);
	let mut grid_next = grid.clone();
	
	let mut steps = 0;
	loop {
		grid_next.fill(b'.');
		let mut count = 0;
		count += advance(&grid, &mut grid_next, 1, 0, b'>');
		std::mem::swap(&mut grid, &mut grid_next);
		grid_next.fill(b'.');
		count += advance(&grid, &mut grid_next, 0, 1, b'v');
		std::mem::swap(&mut grid, &mut grid_next);
		steps += 1;
		if count <= 0 {
			break
		}
	}

	steps
}

#[cfg(test)]
const EXAMPLE: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

#[test]
fn test() {
	assert_eq!(part1(EXAMPLE), 58);
}