use crate::grid::{Grid, Ve};

#[aoc(part1 = 423, part2 = 1287)]
fn solve<const PART1: bool>(input: &str) -> impl std::fmt::Debug {
	let input = Grid::from_char_grid(input);

	const SLOTS: usize = (b'z' - b'0' + 1) as usize;
	let mut count = [0u8; SLOTS];
	let mut nodes = [Ve::zero(); SLOTS * 8];
	let mut nodes = Grid::from_slice([SLOTS, 8], &mut nodes[..]);
	for (pt, k) in input.filter_enumerate(|&v| v != b'.') {
		let index = (k - b'0') as usize;
		nodes[[index, count[index] as usize]] = pt;
		count[index] += 1;
	}

	let mut antis = Grid::new(input.shape(), false);
	let mut num_antis = 0usize;

	for (slot, count) in count.iter().enumerate() {
		let count = *count as usize;
		for i in 0..count {
			for j in (i + 1)..count {
				let (a, b) = (nodes[[slot, i]], nodes[[slot, j]]);
				let delta = a - b;
				let pairs = [(a, delta), (b, -delta)];
				for (start, delta) in pairs {
					for dist in if PART1 { 1..2 } else { 0usize..99 } {
						if let Some(v) = antis.get_mut(start + delta * Ve::from(dist)) {
							num_antis += (!*v) as usize;
							*v = true;
						} else {
							break;
						}
					}
				}
			}
		}
	}

	num_antis
}

#[aoc(part1 = 14, part2 = 34)]
const EX: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";