use crate::grid::Grid;

fn find_mirror(g: &Grid<&[u8]>, fail_want: usize) -> Option<usize> {
	(1..g.width()).filter(|x| {
		let l_x = x - 1;
		let r_x = x;
		let diff = l_x.min(g.width() - 1 - r_x);
		
		let mut failure = 0;
		for y in 0..g.height() {
			for d in 0..=diff {
				if g[[l_x - d, y]] != g[[r_x + d, y]] {
					failure += 1;
					if failure > fail_want {
						return false;
					}
				}
			}
		}

		return failure == fail_want;
	}).next()
}

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
	solve(input, 0)
}
#[aoc(day13, part2)]
fn part2(input: &str) -> usize {
	solve(input, 1)
}

fn solve(input: &str, fail_want: usize) -> usize {
	input.split("\n\n").map(|grid| {
		let grid = Grid::from_char_grid(grid);
		if let Some(v) = find_mirror(&grid, fail_want) {
			v
		} else if let Some(v) = find_mirror(&grid.as_ref().transposed(), fail_want) {
			v * 100
		} else {
			grid.print_c();
			panic!("");
		}
	}).sum()
}