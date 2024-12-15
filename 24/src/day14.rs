use regex::Regex;
use crate::grid::{Grid, Ve};

#[aoc(part1 = 226548000)]
fn part1(input: &str) -> impl std::fmt::Debug {
	part1_0(input, Ve([101, 103]))
}

fn part1_0(input: &str, size: Ve<2>) -> usize {
	let half = size / Ve::from(2isize);
	let mut quads = [0; 4];
	let re = Regex::new(r"^p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)$").unwrap();
	let mut dbg = Grid::new(size, b'0');
	for line in input.trim().lines() {
		let [pt, v] = re.captures(line)
			.expect(line)
			.iter()
			.skip(1)
			.map(|s| s.unwrap().as_str().parse::<isize>().unwrap())
			.array_chunks()
			.map(|c| Ve(c))
			.next_chunk()
			.unwrap();

		let its = 100isize;
		let new_pt = (pt + v * Ve::from(its)).rem_euclid(size);

		dbg[new_pt] += 1;

		let Ve([x, y]) = new_pt - half;
		if x != 0 && y != 0 {
			quads[(x.signum().max(0) + y.signum().max(0) * 2) as usize] += 1;
		}
	}

	println!();
	for y in 0..size[1] {
		for x in 0..size[0] {
			print!("{}", dbg[[x, y]] as char);
		}
		print!("\n");
	}

	quads.iter().cloned().reduce(|a, b| a * b).unwrap()
}

#[aoc(part2 = 7753)]
fn part2(input: &str) -> isize {
	let re = Regex::new(r"^p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)$").unwrap();
	let size = Ve([101, 103]);
	let bots: Vec<[Ve<2>; 2]> = input.trim().lines().map(|line|
		re.captures(line)
			.expect(line)
			.iter()
			.skip(1)
			.map(|s| s.unwrap().as_str().parse::<isize>().unwrap())
			.array_chunks()
			.map(|c| Ve(c))
			.next_chunk()
			.unwrap()).collect();

	let mut dbg = Grid::new(size, b' ');
	for i in 0isize.. {
		dbg.array.fill(b' ');
		for [pt, v] in bots.iter() {
			dbg[(pt + v * Ve::from(i)).rem_euclid(size)] = b'#';
		}

		// this is by far the dumbest aoc problem I have ever done
		if dbg.find(b'#')
			.filter(|pt| {
				for i in pt[0]..size[0] {
					if dbg[[i, pt[1]]] != b'#' {
						return (i - pt[0]) > 25;
					}
				}
				false
			}).count() > 1 {
			println!("\n\n\n{}\n", i);
			for y in 0..size[1] {
				for x in 0..size[0] {
					print!("{}", dbg[[x, y]] as char);
				}
				print!("\n");
			}

			return i;
		}
	}
	panic!();
}

const EX: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

#[test]
fn test() {
	assert_eq!(part1_0(EX, Ve([11, 7])), 12);
}