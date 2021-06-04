#[aoc(day5, part1)]
fn day5_part1(input: &str) -> usize {
	let plane = &mut [[false; 8]; 128];
	for line in input.trim().split("\n") {
		let mut fb = (0, 128);
		let mut lr = (0, 8);
		for c in line.chars() {
			match c {
				'F' => fb = (fb.0, fb.1 / 2),
				'B' => fb = (fb.0 + (fb.1 / 2), fb.1 / 2),
				'L' => lr = (lr.0, lr.1 / 2),
				'R' => lr = (lr.0 + (lr.1 / 2), lr.1 / 2),
				v => panic!("{}", v),
			}
		}
		if fb.1 != 1 || lr.1 != 1 {
			panic!("{} {:?} {:?}", line, fb, lr);
		}
		plane[fb.0][lr.0] = true;
	}
	for row in plane.iter() {
		for seat in row {
			print!("{}", if *seat { "x" } else { " " });
		}
		println!();
	}
	plane
		.iter()
		.enumerate()
		.flat_map(|(row_no, row)| {
			row
				.iter()
				.enumerate()
				.filter(|(_, &v)| v)
				.map(move |(seat, _)| seat + (row_no * 8))
		})
		.max()
		.unwrap()
}
