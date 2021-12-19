use std::collections::HashSet;

type Pos = [isize; 3];
#[derive(Copy, Clone, Debug, Default)]
struct SingleAxisMap {
	sign: i8,
	axis: u8,
}
type AxisMap = [SingleAxisMap; 3];

#[derive(Copy, Clone, Debug)]
struct Location {
	pos: Pos,
	axis_map: AxisMap,
}

fn map_axis(p: &Pos, map: &AxisMap) -> Pos {
	[
		map[0].sign as isize * p[map[0].axis as usize],
		map[1].sign as isize * p[map[1].axis as usize],
		map[2].sign as isize * p[map[2].axis as usize],
	]
}

fn offset(mut a: Pos, b: &Pos) -> Pos {
	a[0] += b[0];
	a[1] += b[1];
	a[2] += b[2];
	a
}

impl Location {
	fn translate(&self, p: &Pos) -> Pos {
		let p = map_axis(p, &self.axis_map);
		offset(p, &self.pos)
	}
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
	let AXIS_MAP = [
		//-x-y-z
		0x000102,
		0x011002,
		0x101102,
		0x110002,
		0x020110,
		0x011210,
		0x121110,
		0x110210,
		0x021011,
		0x101211,
		0x120011,
		0x000211,
		0x021100,
		0x111200,
		0x120100,
		0x010200,
		0x020001,
		0x001201,
		0x121001,
		0x100201,
		0x100112,
		0x010012,
		0x001112,
		0x111012,
	].map(|v| [16, 8, 0].map(|s| {
		let v = v >> s;
		SingleAxisMap{
			sign: if v & 0x10 == 0 {1} else {-1},
			axis: (v & 0xF) as u8,
		}
	}));

	let scanners: Vec<HashSet<Pos>> = input.trim().split("\n\n").map(|block| {
		 block.lines().skip(1).map(|line| {
			let mut l = line.split(",").map(|x| x.parse().unwrap());
			[l.next().unwrap(), l.next().unwrap(), l.next().unwrap()]
		 }).collect()
	}).collect();

	let mut scanner_locations: Vec<Option<Location>> = scanners.iter().map(|_| None).collect();
	let mut with_unfound_neighbors: Vec<usize> = Vec::new();
	scanner_locations[0] = Some(Location {
		pos: [0, 0, 0],
		axis_map: AXIS_MAP[0],
	});
	with_unfound_neighbors.push(0);

	while let Some(i) = with_unfound_neighbors.pop() {
		let src_to_real = &scanner_locations[i].unwrap();
		let src_scanner = &scanners[i];
	'finding:
		for (tsi, loc) in scanner_locations.iter_mut()
			.enumerate()
			.filter(|(_, x)| x.is_none())  {
				let tst_scanner = &scanners[tsi];
				for src_loc_o in src_scanner {
					let src_loc = src_to_real.translate(src_loc_o);
					for tst_loc_o in tst_scanner {
						for axis in &AXIS_MAP {
							let tst_loc = map_axis(tst_loc_o, axis);
							let tst_to_real = Location {
								axis_map: *axis,
								pos: [
									src_loc[0] - tst_loc[0],
									src_loc[1] - tst_loc[1],
									src_loc[2] - tst_loc[2],
								],
							};
							let mut rev_map: AxisMap = Default::default();
							for (s, d) in axis.iter().enumerate() {
								rev_map[d.axis as usize] = SingleAxisMap{
									axis: s as u8,
									sign: d.sign,
								};
							}
							let real_to_tst = tst_to_real.pos.map(|x| -x);
							{
								let test = [1, 2, 3];
								assert_eq!(test, map_axis(&map_axis(&test, axis), &rev_map));
								assert_eq!(tst_to_real.translate(tst_loc_o), src_to_real.translate(src_loc_o));
								assert_eq!(*tst_loc_o, map_axis(&offset(src_to_real.translate(src_loc_o), &real_to_tst), &rev_map), "{:?} {:?}", src_loc_o, tst_loc_o);
							}
							let shared_pts = src_scanner.iter().filter(|&sp| {
								let stp = map_axis(&offset(src_to_real.translate(sp), &real_to_tst), &rev_map);
								tst_scanner.contains(&stp)
							}).count();
							assert!(shared_pts >= 1, "{}", shared_pts);
							if shared_pts >= 12 {
								*loc = Some(tst_to_real);
								with_unfound_neighbors.push(tsi);
								continue 'finding;
							}
						}
					}
				}
			}
	}

	assert!(scanner_locations.iter().all(|l| l.is_some()));

	let mut real_locs = HashSet::new();
	for (si, scanner) in scanners.iter().enumerate() {
		let loc = &scanner_locations[si].unwrap();
		for beacon in scanner {
			real_locs.insert(loc.translate(beacon));
		}
	}

	let pt2: isize = scanner_locations.iter().flat_map(|a| scanner_locations.iter().map(|b| {
		let a = a.unwrap();
		let b = b.unwrap();
		(0..3).map(|i| (a.pos[i] - b.pos[i]).abs()).sum()
	})).max().unwrap();
	println!("part2: {}", pt2);

	real_locs.len()
}