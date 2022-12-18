use std::collections::{HashMap, VecDeque};

use crate::prelude::*;

#[aoc(day18, part1)]
fn day18_part1(input: &str) -> usize {
	let cubes = input.lines().map(|l| l.split(",").must_parse::<isize>().take_n::<3>().unwrap());
	let mut edges: [HashMap<[isize; 3], usize>; 3] = [HashMap::new(), HashMap::new(), HashMap::new()];

	for cube in cubes {
		for dx in 0..2 {
			*edges[0].entry([cube[0] + dx, cube[1], cube[2]]).or_insert(0) += 1;
		}
		for dy in 0..2 {
			*edges[1].entry([cube[0], cube[1] + dy, cube[2]]).or_insert(0) += 1;
		}
		for dz in 0..2 {
			*edges[2].entry([cube[0], cube[1], cube[2] + dz]).or_insert(0) += 1;
		}
	}


	edges.iter()
		.flat_map(|e| e.values())
		.filter(|&&e| e == 1)
		.count()
}

fn idx(off: &[isize; 3], size: &[isize; 3], point: &[isize; 3]) -> usize {
	(((((point[2] - off[2]) * size[1]) + point[1] - off[1]) * size[0]) + (point[0] - off[0])) as usize
}

fn adj1(pt: isize, size: isize) -> impl Iterator<Item=isize> {
	const ADJ1: [isize; 2] = [-1, 1];
	ADJ1.into_iter().map(move |d| pt + d).filter(move |&p| p >= 0 && p < size)
}

fn adj(pt: [isize; 3], size: [isize; 3]) -> impl Iterator<Item=[isize; 3]> {
	let [x, y, z] = pt;
	adj1(pt[0], size[0]).map(move |x| [x, y, z])
		.chain(adj1(pt[1], size[1]).map(move |y| [x, y, z]))
		.chain(adj1(pt[2], size[2]).map(move |z| [x, y, z]))
}

#[aoc(day18, part2)]
fn day18_part2(input: &str) -> usize {
	let cubes = input.lines()
		.map(|l| l.split(",").must_parse::<isize>().take_n::<3>().unwrap())
		.collect::<Vec<_>>();

	let min = cubes.iter().cloned().reduce(|a, b| [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])]).unwrap().map(|v| v - 1);
	let max = cubes.iter().cloned().reduce(|a, b| [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]).unwrap().map(|v| v + 2);

	let off = min;
	let size = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];

	let mut grid = vec![0u8; (size[0] * size[1] * size[2]) as usize];
	for cube in cubes {
		grid[idx(&off, &size, &cube)] = 1;
	}

	let mut todo = VecDeque::new();
	todo.push_back([0, 0, 0]);
	while let Some(pt) = todo.pop_front() {
		for pt in adj(pt, size) {
			let index = idx(&[0, 0, 0], &size, &pt);
			if grid[index] == 0 {
				grid[index] = 2;
				todo.push_back(pt);
			}
		}
	}

	let mut count = 0;
	for x in 0..size[0] {
		for y in 0..size[1] {
			for z in 0..size[2] {
				if (grid[idx(&[0, 0, 0], &size, &[x, y, z])] == 1) {
					count += adj([x, y, z], size).filter(|pt| grid[idx(&[0, 0, 0], &size, &pt)] == 2).count();
				}
			}
		}
	}

	count
}

#[cfg(test)]
const EXAMPLE_1: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

#[test]
fn test() {
	assert_eq!(day18_part1(EXAMPLE_1), 64);
	assert_eq!(day18_part2(EXAMPLE_1), 58);
}