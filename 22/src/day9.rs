use crate::prelude::*;
use std::collections::HashSet;

#[aoc(day9, part1)]
fn day9_part1(input: &str) -> usize {
	solve(input, 2)
}

#[aoc(day9, part2)]
fn day9_part2(input: &str) -> usize {
	solve(input, 10)
}


fn solve(input: &str, size: usize) -> usize {
	let mut visited = HashSet::<(isize, isize)>::new();

	let mut knots = vec![(0, 0); size];
	visited.insert(knots[0]);

	for mov in input.lines() {
		let [dir, mov] = mov.split(" ").take_n::<2>().unwrap();
		let mov = mov.parse::<isize>().unwrap();

		for _ in 0..mov {
			let head = &mut knots[0];
			match dir {
				"R" => head.0 += 1,
				"L" => head.0 -= 1,
				"U" => head.1 -= 1,
				"D" => head.1 += 1,
				v => panic!("{}", v),
			}

			let mut head = *head;
			for tail in &mut knots[1..] {
				let delta = (head.0 - tail.0, head.1 - tail.1);
				if delta.0.abs() >= 2 || delta.1.abs() >= 2 {
					tail.0 += delta.0.signum();
					tail.1 += delta.1.signum();
				}

				head = *tail;
			}

			visited.insert(*knots.last().unwrap());
		}
	}

	visited.len()
}


#[cfg(test)]
const EXAMPLE_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[cfg(test)]
const EXAMPLE_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

#[test]
fn test() {
	assert_eq!(day9_part1(EXAMPLE_1), 13);
	assert_eq!(day9_part2(EXAMPLE_1), 1);
	assert_eq!(day9_part2(EXAMPLE_2), 36);
}