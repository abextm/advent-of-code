#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
	let g = crate::grid::Grid::from_char_grid(input);
	let g = &g;

	let digits = b'0'..=b'9';
	let digits = &digits;

	g.filter_enumerate(|&v| v != b'.' && !digits.contains(&v))
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
	let g = crate::grid::Grid::from_char_grid(input);
	let g = &g;

	let digits = b'0'..=b'9';
	let digits = &digits;

	g.filter_enumerate(|&v| v == b'*')
		.filter_map(|(x, y, _v)| {
			let adj_numbers = g.adjacent8(x, y)
				.filter(|(_x, _y, v)| digits.contains(&v))
				.map(|(x, y, _v)| ((0..=x).rev()
					.take_while(move |x| digits.contains(&g[[*x, y]]))
					.last()
					.unwrap(), y));
			let mut i = 0;
			let mut out: [Option<(usize, usize)>; 2] = [None; 2];
			for adj in adj_numbers {
				if i > 0 {
					if let Some(_) = out.iter().find(|&&x| x == Some(adj)) {
						continue
					}
				}
				if i >= out.len() {
					return None
				}
				out[i] = Some(adj);
				i += 1;
			}

			if let [Some(_), Some(_)] = &out {
				Some([out[0].unwrap(), out[1].unwrap()])
			} else {
				None
			}
		})
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