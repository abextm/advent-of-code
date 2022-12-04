use crate::prelude::*;

#[aoc(day4, part1)]
fn day4_part1(input: &str) -> usize {
	input.trim().lines().filter(|l| {
		let [lhs, rhs] = l.split(",").map(|x| {
			x.split("-").must_parse::<usize>().take_n::<2>().unwrap()
		})
		.take_n::<2>().unwrap();

		lhs[0] <= rhs[0] && lhs[1] >= rhs[1] || rhs[0] <= lhs[0] && rhs[1] >= lhs[1]
	}).count()
}

#[aoc(day4, part2)]
fn day4_part2(input: &str) -> usize {
	input.trim().lines().filter(|l| {
		let [mut lhs, mut rhs] = l.split(",").map(|x| {
			x.split("-").must_parse::<usize>().take_n::<2>().unwrap()
		})
		.take_n::<2>().unwrap();

		if lhs[0] > rhs[0] {
			std::mem::swap(&mut lhs, &mut rhs);
		}

		lhs[1] >= rhs[0]
		
	}).count()
}