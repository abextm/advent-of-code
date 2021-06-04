use crate::grid::Grid;

#[aoc(day11, part1)]
fn day11_part1(input: &str) -> usize {
	let mut g = Grid::new(input);
	let mut g2 = g.clone();
	let mut changed = true;
	while changed {
		std::mem::swap(&mut g, &mut g2);
		changed = false;
		for (x, y, v) in g.iter() {
			match v as char {
				'L'
					if g
						.adjacent8(x, y)
						.filter(|&(_, _, x)| x == '#' as u8)
						.count() == 0 =>
				{
					changed = true;
					g2.set(x, y, '#' as u8);
				}
				'#'
					if g
						.adjacent8(x, y)
						.filter(|&(_, _, x)| x == '#' as u8)
						.count() >= 4 =>
				{
					changed = true;
					g2.set(x, y, 'L' as u8);
				}
				_ => g2.set(x, y, v),
			}
		}
	}

	g.map.iter().filter(|&&x| x == '#' as u8).count()
}

#[aoc(day11, part2)]
fn day11_part2(input: &str) -> usize {
	let mut g = Grid::new(input);
	let mut g2 = g.clone();
	let mut changed = true;
	while changed {
		std::mem::swap(&mut g, &mut g2);
		changed = false;
		for (x, y, v) in g.iter() {
			match v as char {
				'L'
					if g
						.line_of_sight8(x, y, |x| x != '.' as u8)
						.filter(|&(_, _, x)| x == '#' as u8)
						.count() == 0 =>
				{
					changed = true;
					g2.set(x, y, '#' as u8);
				}
				'#'
					if g
						.line_of_sight8(x, y, |x| x != '.' as u8)
						.filter(|&(_, _, x)| x == '#' as u8)
						.count() >= 5 =>
				{
					changed = true;
					g2.set(x, y, 'L' as u8);
				}
				_ => g2.set(x, y, v),
			}
		}
	}

	g.map.iter().filter(|&&x| x == '#' as u8).count()
}
