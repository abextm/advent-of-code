#[aoc(day24, part1)]
fn day24_part1(input: &str) -> usize {
	let mut tiles = std::collections::HashSet::new();
	for line in input.trim().split("\n") {
		let mut inp = line.chars();
		let mut coord = (0, 0, 0);
		loop {
			let c = match inp.next() {
				Some(c) => c,
				None => break,
			};
			match c {
				'e' => coord = add3(coord, (-1, 1, 0)),
				'w' => coord = add3(coord, (1, -1, 0)),
				'n' => match inp.next() {
					Some('e') => coord = add3(coord, (0, 1, -1)),
					Some('w') => coord = add3(coord, (1, 0, -1)),
					v => panic!("{:?}", v),
				},
				's' => match inp.next() {
					Some('e') => coord = add3(coord, (-1, 0, 1)),
					Some('w') => coord = add3(coord, (0, -1, 1)),
					v => panic!("{:?}", v),
				},
				v => panic!("{:?}", v),
			}
		}
		if !tiles.insert(coord) {
			tiles.remove(&coord);
		}
	}
	tiles.len()
}

#[aoc(day24, part2)]
fn day24_part2(input: &str) -> usize {
	let mut tiles = std::collections::HashSet::new();
	for line in input.trim().split("\n") {
		let mut inp = line.chars();
		let mut coord = (0, 0, 0);
		loop {
			let c = match inp.next() {
				Some(c) => c,
				None => break,
			};
			match c {
				'e' => coord = add3(coord, (-1, 1, 0)),
				'w' => coord = add3(coord, (1, -1, 0)),
				'n' => match inp.next() {
					Some('e') => coord = add3(coord, (0, 1, -1)),
					Some('w') => coord = add3(coord, (1, 0, -1)),
					v => panic!("{:?}", v),
				},
				's' => match inp.next() {
					Some('e') => coord = add3(coord, (-1, 0, 1)),
					Some('w') => coord = add3(coord, (0, -1, 1)),
					v => panic!("{:?}", v),
				},
				v => panic!("{:?}", v),
			}
		}
		if !tiles.insert(coord) {
			tiles.remove(&coord);
		}
	}

	let dirs = [
		(-1, 1, 0),
		(1, -1, 0),
		(0, 1, -1),
		(1, 0, -1),
		(-1, 0, 1),
		(0, -1, 1),
	];

	for i in 0..100 {
		let mut tiles2 = tiles.clone();
		for coord in tiles.iter() {
			let mut asib = 0;
			for dir in dirs.iter() {
				let apt = add3(*coord, *dir);
				if tiles.contains(&apt) {
					asib += 1;
				} else {
					let mut bsib = 0;
					for dir in dirs.iter() {
						let bpt = add3(apt, *dir);
						if tiles.contains(&bpt) {
							bsib += 1;
						}
					}

					if bsib == 2 {
						tiles2.insert(apt);
					}
				}
			}
			if asib == 0 || asib > 2 {
				tiles2.remove(&coord);
			}
		}
		tiles = tiles2;
		println!("{} {}", i+1, tiles.len());
	}

	tiles.len()
}

fn add3(a: (isize, isize, isize), b: (isize, isize, isize)) -> (isize, isize, isize) {
	let v = (a.0 + b.0, a.1 + b.1, a.2 + b.2);
	if (v.0 + v.1 + v.2) != 0 {
		panic!("{:?}", v);
	}
	v
}
