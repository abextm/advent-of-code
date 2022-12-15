use std::cmp::Reverse;

use crate::prelude::*;
use regex::Regex;

fn dist(a: &[isize; 2], b: &[isize; 2]) -> isize {
	(a[0] - b[0]).abs() + (a[1] - b[1]).abs()
}

struct Sensor {
	pos: [isize; 2],
	bcn: [isize; 2],
}

impl Sensor {
	fn edges_at_y(&self, y: isize) -> Option<[isize; 2]> {
		let dist = dist(&self.pos, &self.bcn);
		let dx = dist - (self.pos[1] - y).abs();
		if dx < 0 {
			None
		} else {
			Some([self.pos[0] - dx, self.pos[0] + dx])
		}
	}
}

#[aoc_generator(day15)]
fn parse(s: &str) -> Vec<Sensor> {
	let re = Regex::new(r"Sensor at x=([-0-9]+), y=([-0-9]+): closest beacon is at x=([-0-9]+), y=([-0-9]+)").unwrap();
	s.lines().map(|l| {
		let [px, py, bx, by] = re.captures(l).unwrap().iter().skip(1).map(|x| x.unwrap().as_str().parse::<isize>().unwrap()).take_n::<4>().unwrap();
		Sensor {
			pos: [px, py],
			bcn: [bx, by],
		}
	}).collect()
}

#[aoc(day15, part1)]
fn day15_part1(input: &[Sensor]) -> isize {
	part1(input, 2000000)
}

fn part1(sensors: &[Sensor], y: isize) -> isize {
	let mut blocks = 0;
	let mut x = isize::MIN;
	while let Some([l, h]) = sensors.iter()
		.filter_map(|x| x.edges_at_y(y))
		.filter(|&[_l, h]| h > x)
		.min_by_key(|&[l, h]| (l.max(x), Reverse(h))) {
			blocks += h - l.max(x);
			x = h;
		}

	blocks
}

#[aoc(day15, part2)]
fn day15_part2(input: &[Sensor]) -> isize {
	let [x, y] = part2(input, 4000000);
	x * 4000000 + y
}

fn part2(sensors: &[Sensor], max: isize) -> [isize; 2] {
	let min = 0;

	for y in min..=max {
		let mut x = min;
		while let Some([_l, h]) = sensors.iter()
			.filter_map(|x| x.edges_at_y(y))
			.filter(|&[l, h]| h > x && x >= l)
			.max_by_key(|&[_l, h]| h) {
				x = h;
			}
		if x < max {
			return [x + 1, y];
		}
	}

	panic!();
}

#[cfg(test)]
const EXAMPLE_1: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

#[test]
fn test() {
	let sensors = parse(EXAMPLE_1);
	assert_eq!(part1(&sensors, 10), 26);
	assert_eq!(part2(&sensors, 20), [14, 11]);
}