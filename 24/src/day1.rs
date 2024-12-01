use std::collections::HashMap;

#[aoc(part1=1388114)]
fn solve(input: &str) -> impl std::fmt::Debug {
	let mut a: Vec<i32> = Vec::new();
	let mut b = a.clone();
	for l in input.trim().lines() {
		let [av, bv] = l.split("   ").next_chunk().unwrap();
		a.push(av.parse().expect(av));
		b.push(bv.parse().expect(bv));
	}

	a.sort();
	b.sort();

	a.iter().zip(b.iter())
		.map(|(a, b)| (a - b).abs())
		.sum::<i32>()
}

#[aoc(part2=23529853)]
fn part2(input: &str) -> impl std::fmt::Debug {
	let mut a = Vec::<i32>::new();
	let mut b = HashMap::<i32, i32>::new();
	for l in input.trim().lines() {
		let [av, bv] = l.split("   ").next_chunk().unwrap();
		a.push(av.parse().expect(av));
		let bv = bv.parse().expect(bv);
		*b.entry(bv).or_default() += 1;
	}

	a.iter().map(|v| v * b.get(v).cloned().unwrap_or(0)).sum::<i32>()
}

#[aoc(part1=11, part2=31)]
const EX: &str = "3   4
4   3
2   5
1   3
3   9
3   3";