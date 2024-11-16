use crate::itersort::IterSortEx;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;

#[aoc(part1=263)]
fn day10_part1(input: &str) -> usize {
	let (map, _) = decode(input);
	let (_, out) = find_best(&map);
	out
}

#[aoc(part2=1110)]
fn day10_part2(input: &str) -> usize {
	let (map, _) = decode(input);
	let (best, _) = find_best(&map);
	let l = laser(&map, best)[199];
	l.0 * 100 + l.1
}

fn decode(input: &str) -> (Vec<(usize, usize)>, Option<(usize, usize)>) {
	let mut out = Vec::with_capacity(input.len());
	let mut height = 0;
	let mut current = (0, 0);
	let mut spec = None;
	for c in input.chars() {
		if c == '\n' {
			height += 1;
			current = (0, height);
		} else {
			if c == 'x' {
				spec = Some(current);
			}
			if c != '.' {
				out.push(current)
			}
			current.0 += 1;
		}
	}
	(out, spec)
}

fn angle(i: (usize, usize), c: (usize, usize)) -> Option<u32> {
	if i == c {
		None
	} else {
		let angle = (i.1 as f64 - c.1 as f64).atan2(i.0 as f64 - c.0 as f64);
		Some((((angle / std::f64::consts::PI * 10000f64) + 25000f64) % 20000f64) as u32)
	}
}

fn find_best(map: &Vec<(usize, usize)>) -> ((usize, usize), usize) {
	map
		.iter()
		.map(|&c| {
			let set = map
				.iter()
				.filter_map(|&i| angle(i, c))
				.collect::<HashSet<_>>();
			(c, set.len())
		})
		.max_by_key(|(_, v)| *v)
		.unwrap()
}

fn laser(map: &Vec<(usize, usize)>, center: (usize, usize)) -> Vec<(usize, usize)> {
	let mut angles = HashMap::<u32, Vec<(usize, usize)>>::new();
	for &x in map {
		if let Some(a) = angle(x, center) {
			angles.entry(a).or_insert(Vec::new()).push(x);
		}
	}
	let (send, recv) = mpsc::channel();
	for (_, t) in angles.iter_mut().sort_by_key(|(k, _)| *k) {
		t.sort_by_key(|&n| {
			let v = (
				n.0 as isize - center.0 as isize,
				n.1 as isize - center.1 as isize,
			);
			!(v.0 * v.0 + v.1 * v.1)
		});
		send.send(t).unwrap();
	}
	let mut out = Vec::new();
	loop {
		match recv.try_recv() {
			Ok(v) => {
				let val = v.pop().unwrap();
				if !v.is_empty() {
					send.send(v).unwrap();
				}
				out.push(val);
			}
			Err(_) => break,
		}
	}

	out
}

#[test]
fn test_angle() {
	assert_eq!(angle((1, 0), (1, 1)).unwrap(), 0);
	assert_eq!(angle((2, 1), (1, 1)).unwrap(), 5000);
	assert_eq!(angle((1, 2), (1, 1)).unwrap(), 10000);
	assert_eq!(angle((0, 1), (1, 1)).unwrap(), 15000);
	assert_eq!(angle((0, 0), (1, 1)).unwrap(), 17500);
}

#[test]
fn test1() {
	let (map, good) = decode(
		".#..#
.....
#####
....#
...x#",
	);
	assert_eq!(find_best(&map), (good.unwrap(), 8));
}

#[test]
fn test3() {
	let (map, good) = decode(
		"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...x..#.
.#....####",
	);
	assert_eq!(find_best(&map), (good.unwrap(), 33));
}

#[test]
fn test4() {
	let (map, good) = decode(
		"#.#...#.#.
.###....#.
.x....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.",
	);
	assert_eq!(find_best(&map), (good.unwrap(), 35));
}
#[test]
fn test5() {
	let (map, good) = decode(
		".#..#..###
####.###.#
....###.#.
..###.x#.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..",
	);
	assert_eq!(find_best(&map), (good.unwrap(), 41));
}

#[test]
fn easy() {
	let (map, center) = decode(
		"###
#x#
###",
	);
	assert_eq!(
		laser(&map, center.unwrap()),
		[
			(1, 0),
			(2, 0),
			(2, 1),
			(2, 2),
			(1, 2),
			(0, 2),
			(0, 1),
			(0, 0),
		]
	);
}

#[test]
fn test6() {
	let (map, good) = decode(
		".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.####x#####...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
	);
	let best = find_best(&map);
	assert_eq!(best, (good.unwrap(), 210));

	let laser = laser(&map, best.0);
	assert_eq!(laser[0..3], [(11, 12), (12, 1), (12, 2)]);
	assert_find(&laser, (12, 8), 9);
	assert_find(&laser, (16, 0), 19);
	assert_find(&laser, (16, 9), 49);
	assert_find(&laser, (10, 16), 99);
	assert_find(&laser, (9, 6), 198);
	assert_find(&laser, (8, 2), 199);
	assert_find(&laser, (10, 9), 200);
	assert_find(&laser, (11, 1), 298);
}

#[cfg(test)]
fn assert_find(laser: &Vec<(usize, usize)>, s: (usize, usize), index: usize) {
	let f = laser.iter().position(|&x| x == s).unwrap();
	assert_eq!(f, index)
}

#[test]
fn test_pt2() {
	let (map, center) = decode(
		".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....x...###..
..#.#.....#....##",
	);
	assert_eq!(*laser(&map, center.unwrap()).last().unwrap(), (14, 3));
}
