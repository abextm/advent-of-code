use crate::grid;

#[derive(Debug)]
struct Tile {
	grid: grid::Grid,
	id: i64,
	sides: [Option<(i64, i64)>; 4],
}

#[aoc(day20, part1)]
fn day20_part1(input: &str) -> usize {
	let mut tiles = std::collections::HashMap::<_, Tile>::new();
	let mut side_tiles = std::collections::HashMap::<_, Vec<_>>::new();
	let tile_matcher = regex::Regex::new("^Tile ([0-9]+):\n([#.\n]+)$").unwrap();

	let sides = [
		// x  x2  y  y2  i
		((0, 1, 0, 0), 0 as i64), // >
		((1, 1, 0, 1), 1),        // v
		((0, 1, 1, 1), 2),        // <
		((0, 0, 0, 1), 3),        // ^
	];

	for tile in input.trim().split("\n\n") {
		let c = tile_matcher.captures(tile).expect(tile);
		let id = c[1].parse::<i64>().unwrap();
		let mut grid = grid::Grid::new(&c[2]);

		for &((x1, x2, y1, y2), rot) in sides.iter() {
			let (w, h) = (grid.width - 1, grid.height - 1);
			let (x1, x2, y1, y2) = (x1 * w, x2 * w, y1 * h, y2 * h);
			let mut side = Vec::new();
			for x in x1..=x2 {
				for y in y1..=y2 {
					side.push(grid.get(x, y));
				}
			}
			let flip = if rot < 2 { (0, 4) } else { (4, 0) };
			side_tiles
				.entry(side.clone())
				.or_default()
				.push((id, rot + flip.0));
			side.reverse();
			side_tiles.entry(side).or_default().push((id, rot + flip.1));
		}

		tiles.insert(
			id,
			Tile {
				grid,
				id,
				sides: [None; 4],
			},
		);
	}

	let mut sides = side_tiles
		.iter()
		.filter(|(_, v)| v.len() > 1)
		.collect::<Vec<_>>();
	sides.sort_by_key(|(k, _)| k.len());

	for (key, ops) in sides.iter() {
		if ops.len() != 2 {
			panic!("hm");
		}
		let (mut a, mut b) = (ops[0], ops[1]);
		for i in 0..2 {
			let ts = &mut tiles.get_mut(&a.0).unwrap().sides[(a.1 % 4) as usize];
			if a.1 >= 4 {
				a.1 ^= 4;
				b.1 ^= 4;
			}
			match ts {
				None => (),
				Some(v) if *v == b => (),
				Some(v) => panic!("{:?} {:?}", b, v),
			};
			*ts = Some((b.0, b.1));

			std::mem::swap(&mut a, &mut b);
		}
	}

	let mut corners = tiles
		.iter()
		.filter(|(_, tile)| tile.sides.iter().filter(|x| x.is_none()).count() == 2)
		.collect::<Vec<_>>();

	let dim = (tiles.len() as f64).sqrt() as usize;
	//let mut row_start = tiles.get(&1951).unwrap();
	let mut tiles_sort = tiles.iter().collect::<Vec<_>>();
	tiles_sort.sort_by_key(|&(id, _)| -id);
	let mut row_start = tiles_sort
		.into_iter()
		.map(|(_, tile)| tile)
		.filter(|tile| tile.sides[2] == None && tile.sides[3] == None)
		.next()
		.unwrap();
	let mut row_start_rotation = Rotation {
		fx: false,
		fy: true,
		rot: 0,
	};
	let width = 9;
	let offset = 1;
	let realwidth = 8;
	let range = 1..9;
	let mut out = grid::Grid::blank(realwidth * dim, realwidth * dim);
	for row in 0..dim {
		let mut tile = row_start;
		let mut rot = row_start_rotation;

		for col in 0..dim {
			for x in range.clone() {
				for y in range.clone() {
					let (mut rx, mut ry) = match rot.rot {
						0 => (x, y),                 // >
						1 => (width - y, x),         // v
						2 => (width - x, width - y), // <
						3 => (y, width - x),         // ^
						_ => panic!(),
					};
					if rot.fx {
						rx = width - rx;
					}
					if rot.fy {
						ry = width - ry;
					}
					let val = tile.grid.get(x, y);
					out.set(rx + (col * realwidth) - offset, ry + (row * realwidth) - offset, val);
				}
			}
/*
			out.set(
				2 + (col * realwidth),
				2 + (row * realwidth),
				'0' as u8 + (col % 10) as u8,
			);
			out.set(
				4 + (col * realwidth),
				2 + (row * realwidth),
				'0' as u8 + (row % 10) as u8,
			);
			out.set(
				2 + (col * realwidth),
				3 + (row * realwidth),
				'0' as u8 + rot.rot as u8,
			);
			out.set(
				3 + (col * realwidth),
				3 + (row * realwidth),
				if rot.fx { 'x' } else { ' ' } as u8,
			);
			out.set(
				4 + (col * realwidth),
				3 + (row * realwidth),
				if rot.fy { 'y' } else { ' ' } as u8,
			);
			out.set(2 + (col * realwidth), 4 + (row * realwidth), '0' as u8 + (tile.id / 1000 % 10) as u8);
			out.set(3 + (col * realwidth), 4 + (row * realwidth), '0' as u8 + (tile.id / 100 % 10) as u8);
			out.set(4 + (col * realwidth), 4 + (row * realwidth), '0' as u8 + (tile.id / 10 % 10) as u8);
			out.set(5 + (col * realwidth), 4 + (row * realwidth), '0' as u8 + (tile.id / 1 % 10) as u8);
*/
			tile = match tile.sides[rot.side(1)] {
				Some((id, rot2)) => {
					rot = add_rotation(rot, 1, rot2);
					tiles.get(&id).unwrap()
				}
				None => break,
			}
		}

		row_start = match row_start.sides[row_start_rotation.side(2)] {
			Some((id, rot)) => {
				row_start_rotation = add_rotation(row_start_rotation, 2, rot);
				tiles.get(&id).unwrap()
			}
			None => break,
		}
	}

	let mut monster = grid::Grid::new("                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ");
	let mut monsters = Vec::new();
	for _ in 0..4 {
		monsters.push(monster.clone());
		monsters.push(monster.flipleftright());
		monster = monster.rotate();
	}
	let monsters = monsters.into_iter().map(|g| {
		g.iter().partition::<Vec<_>,_>(|(_, _, v)| *v == '#' as u8)
	}).collect::<Vec<_>>();

	let mut turb = 0;
	for (x, y, v) in out.iter() {
		if v == '#' as u8 {
			turb += 1;
		}

		for monster in monsters.iter() {
			if monster.0.iter().all(|(dx, dy, v)| out.get_safe((x + dx) as isize, (y + dy) as isize) == Some(*v)) {
				turb -= monster.0.len();
			}
		}
	}

	out.print();

	println!("Part1: {}", corners.iter().map(|x|x.0).product::<i64>());
	turb
}

#[derive(Copy, Clone, Debug)]
struct Rotation {
	fx: bool,
	fy: bool,
	rot: i64,
}

impl Rotation {
	fn side(&self, rot: usize) -> usize {
		let mut side = 4 - self.rot as usize;
		if match rot % 2 {
			0 => self.fy,
			1 => self.fx,
			_ => panic!(),
		} {
			side += 2;
		}
		side += rot;
		side % 4
	}
}

fn add_rotation(mut existing: Rotation, direction: i64, new: i64) -> Rotation {
	match direction {
		1 => {
			existing.rot = 3 - new % 4;
			existing.fy ^= new < 4;
			if existing.fx {
				existing.rot = (2 + existing.rot) % 4;
			}
		},
		2 => {
			existing.rot = (2 - (new % 4) + 4)%4;
			existing.fx ^= new < 4;	
		},
		_ => panic!(),
	}
	existing
}
