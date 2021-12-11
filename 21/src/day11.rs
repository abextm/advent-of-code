use crate::grid::Grid;

fn inc(grid: &mut Grid<u8>, flashed: &mut Grid<bool>, x: usize, y: usize) {
	let v = grid.at_mut(x, y);
	*v += 1;
	let v = *v;
	if v > 9 && !flashed.get(x, y){
		flashed.set(x, y, true);
		let adj = grid.adjacent8(x, y)
			.map(|(x, y, _)| (x, y))
			.collect::<Vec<_>>();
		for (x, y) in adj {
			inc(grid, flashed, x, y);
		}
	}
}

fn advance(grid: &mut Grid<u8>) -> usize {
	let mut flashed = Grid::blank(grid.width, grid.height, false);
	for y in 0..grid.height {
		for x in 0..grid.width {
			inc(grid, &mut flashed, x, y);
		}
	}
	let mut flashes = 0;
	for (x, y, &f ) in flashed.iter() {
		if f {
			flashes+= 1;
			grid.set(x, y, 0);
		}
	}
	flashes
}

#[aoc(day11, part1)]
fn day11_part1(input: &str) -> usize {
	let mut grid = crate::grid::Grid::from_number_grid(input);
	let mut flashes = 0;
	for _ in 0..100 {
		flashes += advance(&mut grid);
	}
	flashes
}
#[aoc(day11, part2)]
fn day11_part2(input: &str) -> usize {
	let mut grid = crate::grid::Grid::from_number_grid(input);
	for i in 1.. {
		let flashes = advance(&mut grid);
		if flashes == grid.width * grid.height {
			return i;
		}
	}
	panic!();
}

#[cfg(test)]
const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

#[test]
fn test_p1() {
	assert_eq!(1656, day11_part1(EXAMPLE));
}