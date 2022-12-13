use std::cmp::Ordering;

use serde::Deserialize;
use crate::prelude::*;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum Entry {
	List(Vec<Entry>),
	Value(usize),
}

fn compare(a: &[Entry], b: &[Entry]) -> std::cmp::Ordering {
	for (a, b) in a.iter().zip(b.iter()) {
		match a.cmp(b) {
			Ordering::Equal => {},
			v => return v,
		};
	}
	a.len().cmp(&b.len())
}

impl std::cmp::Ord for Entry {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match (self, other) {
			(Self::Value(l), Self::Value(r)) => l.cmp(r),
			(&Self::Value(l), Self::List(r)) => compare(&[Entry::Value(l)], r.as_slice()),
			(Self::List(l), &Self::Value(r)) => compare(l.as_slice(), &[Entry::Value(r)]),
			(Self::List(l), Self::List(r)) => compare(l.as_slice(), r.as_slice()),
		}
	}
}
impl PartialOrd for Entry {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[aoc(day13, part1)]
fn day13_part1(input: &str) -> usize {
	input.trim()
		.split("\n\n")
		.enumerate()
		.map(|(i, block)| {
		let [l, r] = block.trim()
			.split("\n")
			.map(|line| serde_json::from_str::<Entry>(&line).unwrap())
			.take_n::<2>()
			.unwrap();
		if l < r {
			i + 1
		} else {
			0
		}
	}).sum()
}

#[aoc(day13, part2)]
fn day13_part2(input: &str) -> usize {
	let mut packets = input.trim()
		.lines()
		.filter(|l| !l.is_empty())
		.map(|line| serde_json::from_str::<Entry>(&line).unwrap())
		.collect::<Vec<_>>();
	
	let keys = [
		Entry::List(vec![Entry::List(vec![Entry::Value(2)])]),
		Entry::List(vec![Entry::List(vec![Entry::Value(6)])]),
	];
	packets.extend_from_slice(&keys);

	packets.sort();
	keys.iter()
		.map(|k| packets.iter().position(|x| x == k).unwrap() + 1)
		.reduce(|a, b| a * b)
		.unwrap()	
}