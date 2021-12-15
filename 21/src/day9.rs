use crate::grid::Grid;
#[aoc(day9, part1)]
fn day9_part1(input: &str) -> usize {
	let grid = Grid::from_number_grid(input);
	grid.iter()
	.filter(|&(x, y, &v)| 
		grid.adjacent4(x, y).all(|(_, _, &ov)| ov > v))
	.map(|(_, _, &v)| (v + 1) as usize)
	.sum::<usize>()
}

#[aoc(day9, part2)]
fn day9_part2(input: &str) -> usize {
	let grid = Grid::from_number_grid(input);
	let mut sums = grid.map(|_, _, &v| if v == 9 {0} else {1});

	for _ in (0..=9).rev() {
		grid.iter().for_each(|(x, y, &v)| {
			if let Some((nx, ny, &nv)) = grid.adjacent4(x, y).min_by_key(|&(_, _, &v)| v) {
				if nv < v {
					sums[[nx, ny]] += sums[[x, y]];
					sums[[x, y]] = 0;
				}
			}
		});
	}
	
	let mut v: Vec<_>  = sums.values_iter().cloned()
		.filter(|&v| v > 0)
		.collect();

	v.sort();
	v.reverse();
	v.iter().take(3)
		.fold(1, |a, b| a * b)
}

#[cfg(test)]
const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

#[test]
fn test_p1() {
	assert_eq!(day9_part1(EXAMPLE), 15)
}
#[test]
fn test_p2() {
	assert_eq!(day9_part2(EXAMPLE), 1134)
}