#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
	let g = crate::grid::Grid::from_str_with_mapper(input, |x| *x);
	let g = &g;

	let digits = b'0'..=b'9';
	let digits = &digits;

	g.iter()
		.filter(|&(_x, _y, &v)| v != b'.' && !digits.contains(&v))
		.flat_map(|(x, y, _v)| g.adjacent8(x, y)
			.filter(|(_x, _y, v)| digits.contains(&v))
			.map(|(x, y, _v)| ((0..=x).rev()
				.take_while(move |x| digits.contains(&g[[*x, y]]))
				.last()
				.unwrap(), y)))
		.collect::<std::collections::HashSet<_>>()
		.into_iter()
		.map(|(x, y)| (x..g.width())
			.map(|x| g[[x, y]])
			.take_while(|v| digits.contains(v))
			.map(|x| x as char)
			.collect::<String>()
			.parse::<usize>()
			.unwrap())
		.sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
	let g = crate::grid::Grid::from_str_with_mapper(input, |x| *x);
	let g = &g;

	let digits = b'0'..=b'9';
	let digits = &digits;

	g.iter()
		.filter(|&(_x, _y, &v)| v == b'*')
		.map(|(x, y, _v)| g.adjacent8(x, y)
			.filter(|(_x, _y, v)| digits.contains(&v))
			.map(|(x, y, _v)| ((0..=x).rev()
				.take_while(move |x| digits.contains(&g[[*x, y]]))
				.last()
				.unwrap(), y))
			.collect::<std::collections::HashSet<_>>())
		.filter(|v| v.len() == 2)
		.map(|vals| vals.iter()
			.map(|&(x, y)| (x..g.width())
				.map(|x| g[[x, y]])
				.take_while(|v| digits.contains(v))
				.map(|x| x as char)
				.collect::<String>()
				.parse::<usize>()
				.unwrap())
			.reduce(|a, b| a * b).unwrap())
		.sum()
}

#[test]
fn test_p1() {
	assert_eq!(4361, part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."));
}