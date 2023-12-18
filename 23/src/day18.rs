type Point = (isize, isize);

struct Instr {
	dir: u8,
	distance: isize,
}

fn add(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
	(a.0 + b.0, a.1 + b.1)
}

fn is_right_turn(a: u8, b: u8) -> bool {
	match (a, b) {
		(b'U', b'R') => true,
		(b'U', b'L') => false,
		(b'R', b'D') => true,
		(b'R', b'U') => false,
		(b'D', b'L') => true,
		(b'D', b'R') => false,
		(b'L', b'U') => true,
		(b'L', b'D') => false,
		_ => panic!(),
	}
}

fn area<I: Iterator<Item=Instr>>(iter: I) -> isize {
	let mut point = (0isize, 0isize);
	iter.map(|ins| {
		let last_point = point;
		let delta = match ins.dir {
			b'R' => (1, 0),
			b'U' => (0, -1),
			b'L' => (-1, 0),
			b'D' => (0, 1),
			_ => panic!(),
		};
		point = add(point, (delta.0 * ins.distance, delta.1 * ins.distance));
		(last_point.0 * point.1) - (last_point.1 * point.0) + ins.distance
	}).sum::<isize>() / 2 + 1
}

#[aoc(day18, part1)]
fn part1(input: &str) -> isize {
	let pattern = regex::Regex::new(r#"(.) ([0-9]+) \(#[0-9a-z]+\)"#).unwrap();
	area(input.trim().lines()
		.map(|line| {
			let caps = pattern.captures(line).unwrap();
			let dir = caps[1].as_bytes()[0];
			let dist: isize = caps[2].parse().unwrap();
			let instr = Instr {
				dir: dir,
				distance: dist,
			};
			instr
		}))
}

#[aoc(day18, part2)]
fn part2(input: &str) -> isize {
	let pattern = regex::Regex::new(r#". [0-9]+ \(#([0-9a-z]{5})([0-4])\)"#).unwrap();
	area(input.trim().lines()
		.map(|line| {
			let caps = pattern.captures(line).unwrap();
			let dir = match caps[2].as_bytes()[0] {
				b'0' => b'R',
				b'1' => b'D',
				b'2' => b'L',
				b'3' => b'U',
				_ => panic!(),
			};
			let dist: isize = isize::from_str_radix(&caps[1], 16).unwrap() as isize;
			let instr = Instr {
				dir: dir,
				distance: dist,
			};
			instr
		}))
}

#[cfg(test)]
const EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

#[test]
fn test_p1() {
	assert_eq!(62, part1(EXAMPLE))
}
#[test]
fn test_p2() {
	assert_eq!(952408144115, part2(EXAMPLE))
}