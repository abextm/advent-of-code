use crate::prelude::*;
use crate::grid::Grid;

#[aoc(day14, part1)]
fn day14_part1(input: &str) -> usize {
	let mut grid = Grid::blank(&(600, 600), 0u8);
	for line in input.lines() {
		let mut points = line.split(" -> ").map(|bit| bit.split(',').must_parse::<usize>().take_n::<2>().unwrap());
		let mut pt = points.next().unwrap();
		for next in points {
			for x in pt[0].min(next[0])..=pt[0].max(next[0]) {
				for y in pt[1].min(next[1])..=pt[1].max(next[1]) {
					grid[[x, y]] = 1;
				}
			}

			pt = next;
		}
	}

	let mut its = 0;
	loop {
		let mut p = [500, 0];
		while p[1] < grid.height() - 1 {
			let next = [p[0], p[1] + 1];
			if grid[next] == 0 {
				p = next;
			} else if grid[[next[0] - 1, next[1]]] == 0 {
				p = [next[0] - 1, next[1]]
			} else if grid[[next[0] + 1, next[1]]] == 0 {
				p = [next[0] + 1, next[1]]
			} else {
				println!("{:?}", p);
				grid[p] = 2;
				break;
			}
		}
		if p[1] >= grid.height() - 1 {
			break;
		}

		its += 1;
	}

	its
}

#[aoc(day14, part2)]
fn day14_part2(input: &str) -> usize {
	let mut grid = Grid::blank(&(700, 200), 0u8);
	let mut maxY = 0;
	for line in input.lines() {
		let mut points = line.split(" -> ").map(|bit| bit.split(',').must_parse::<usize>().take_n::<2>().unwrap());
		let mut pt = points.next().unwrap();
		for next in points {
			maxY = maxY.max(pt[1].max(next[1]));
			for x in pt[0].min(next[0])..=pt[0].max(next[0]) {
				for y in pt[1].min(next[1])..=pt[1].max(next[1]) {
					grid[[x, y]] = 1;
				}
			}

			pt = next;
		}
	}
	let floor = 2 + maxY;
	for x in 0..grid.width() {
		grid[[x, floor]] = 3;
	}

	let mut its = 0;
	loop {
		let mut p = [500, 0];
		if grid[p] != 0{
			break;
		}
		while p[1] < grid.height() - 1 {
			let next = [p[0], p[1] + 1];
			if grid[next] == 0 {
				p = next;
			} else if grid[[next[0] - 1, next[1]]] == 0 {
				p = [next[0] - 1, next[1]]
			} else if grid[[next[0] + 1, next[1]]] == 0 {
				p = [next[0] + 1, next[1]]
			} else {
				println!("{:?}", p);
				grid[p] = 2;
				break;
			}
		}
		if p[1] >= grid.height() - 1 {
			break;
		}

		its += 1;
	}

	grid.print_mapped(|&v| match v {
		0 => ' ',
		1 => '█',
		2 => '.',
		3 => '=',
		_ => panic!(),
	});

	its
}