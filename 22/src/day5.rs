use crate::prelude::*;

#[aoc(day5, part1)]
fn day5_part1(input: &str) -> String {
	solve(input, false)
}

#[aoc(day5, part2)]
fn day5_part2(input: &str) -> String {
	solve(input, true)
}

fn get_mut2<T>(v: &mut [T], ai: usize, bi: usize) -> (&mut T, &mut T) {
	let (a, b) = v.split_at_mut(ai.max(bi));
	let a = &mut a[ai.min(bi)];
	let b = &mut b[0];
	if ai > bi {
		(b, a)
	} else {
		(a, b)
	}
}

fn solve(input: &str, pt2: bool) -> String {
	let [stacks_str, instrs] = input.split("\n\n").take_n::<2>().unwrap();

	let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); stacks_str.find("\n").unwrap() + 1 / 4];

	for stack in stacks_str.lines().rev().skip(1) {
		for i in (1..stack.len()).step_by(4) {
			let c = stack.as_bytes()[i];
			if c != b' ' {
				stacks[i / 4].push(c);
			}
		}
	}
	
	let re = regex::Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
	for line in instrs.lines() {
		let cap = re.captures(line).unwrap();
		let [count, from, to] = cap.iter()
			.skip(1)
			.map(|m| m.unwrap().as_str().parse::<usize>().unwrap())
			.take_n::<3>()
			.unwrap();

		let (from, to) = get_mut2(&mut stacks[..], from - 1, to - 1);
		if pt2 {
			to.extend_from_slice(&from[from.len() - count..]);
			from.truncate(from.len() - count);
		} else {
			for _ in 0..count {
				to.push(from.pop().unwrap());
			}
		}
	}

	stacks.iter()
		.filter_map(|s| s.last())
		.map(|&c| c as char)
		.collect::<String>()
}